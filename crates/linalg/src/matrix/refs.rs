use core::{
    ptr::{self, slice_from_raw_parts, slice_from_raw_parts_mut},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use super::Matrix;
use crate::{packing::Packed, vector::StridedVectorRef};

// This is a hack I came up with after several iterations.  The issue is that in
// Rust only `&` and `&mut` are first class references.  The previous design
// used two objects, `MatrixRef(*const T, usize, usize)` and `MatrixRefMut(*mut
// T, usize, usize)`, but it was suboptimal.  `MatrixRefMut` had to be
// reborrowed and I needed two traits (`Matrix` and `MatrixMut`).  Furthermore,
// in those traits I had to either take `self` by value complicating working
// with other objects or taking it by reference, meaning it'd be a double
// pointer.
//
// Now, `[T]` does have a single `usize` of metadata.  Now, the slice docs
// heavily emphasise the dangers of an invalid reference.  Even creating one is
// instant UB.  The main reason, however, as I understand it, is that Rust
// annotates pointer with `nonnull` and `dereferenceable`.  That doesn't apply
// to the length, though.
//
// Hence the design of `MatrixRef`.  It's an unsized wrapper around `[T]`.  The
// pointer part of the reference always points to the start of a valid
// allocation, so it shouldn't cause UB.  But the length/metadata has been
// repurposed to store number of rows in the high bits and number of columns in
// the low ones.  This means one can create `&MatrixRef` and `&mut MatrixRef`
// with all the ergonomics of a regular reference.
#[repr(transparent)]
pub struct MatrixRef<T>([T]);

impl<T> Matrix<T> for MatrixRef<T> {
    type Row = [T];
    type Column = StridedVectorRef<T>;

    #[cfg(target_pointer_width = "64")]
    fn num_rows(&self) -> usize {
        self.0.len().lower()
    }

    #[cfg(target_pointer_width = "64")]
    fn num_cols(&self) -> usize {
        self.0.len().upper()
    }

    fn row_stride(&self) -> usize {
        self.num_cols()
    }

    unsafe fn at_u(&self, row: usize, col: usize) -> &T {
        let idx = self.index_of(row, col);
        unsafe { &*self.as_ptr().add(idx) }
    }

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.index_of(row, col);
        unsafe { &mut *self.as_mut_ptr().add(idx) }
    }

    unsafe fn row_u(&self, index: usize) -> &[T] {
        unsafe {
            let ptr = self.as_ptr().add(index * self.num_cols());
            from_raw_parts(ptr, self.num_cols())
        }
    }

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut [T] {
        unsafe {
            let ptr = self.as_mut_ptr().add(index * self.num_cols());
            from_raw_parts_mut(ptr, self.num_cols())
        }
    }

    unsafe fn col_u(&self, index: usize) -> &StridedVectorRef<T> {
        unsafe {
            StridedVectorRef::from_raw_parts(
                Matrix::at_u(self, 0, index) as *const T,
                self.num_rows() as u32,
                self.row_stride() as u32,
            )
        }
    }

    unsafe fn col_mut_u(&mut self, index: usize) -> &mut StridedVectorRef<T> {
        unsafe {
            StridedVectorRef::from_raw_parts_mut(
                Matrix::at_mut_u(self, 0, index) as *mut T,
                self.num_rows() as u32,
                self.row_stride() as u32,
            )
        }
    }

    unsafe fn swap_rows_u(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        unsafe {
            let ptr_a = self.at_mut_u(a, 0) as *mut T;
            let ptr_b = self.at_mut_u(b, 0) as *mut T;
            ptr::swap_nonoverlapping(ptr_a, ptr_b, self.num_rows());
        };
    }

    fn for_each(&self, f: impl FnMut(&T)) {
        self.as_slice().iter().for_each(f);
    }

    fn for_each_mut(&mut self, f: impl FnMut(&mut T)) {
        self.as_slice_mut().iter_mut().for_each(f);
    }
}

impl<T> MatrixRef<T> {
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    pub fn num_elements(&self) -> usize {
        self.num_rows() * self.num_cols()
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { from_raw_parts(self.as_ptr(), self.num_elements()) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { from_raw_parts_mut(self.as_mut_ptr(), self.num_elements()) }
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        self.num_cols() * row + col
    }
}

impl<T> MatrixRef<T> {
    #[cfg(target_pointer_width = "64")]
    pub unsafe fn from_raw_parts<'a>(
        ptr: *const T,
        num_rows: u32,
        num_cols: u32,
    ) -> &'a Self {
        let slice =
            slice_from_raw_parts(ptr, usize::from_halves(num_rows, num_cols));
        unsafe { &*(slice as *const Self) }
    }

    #[cfg(target_pointer_width = "64")]
    pub unsafe fn from_raw_parts_mut<'a>(
        ptr: *mut T,
        num_rows: u32,
        num_cols: u32,
    ) -> &'a mut Self {
        let slice = slice_from_raw_parts_mut(
            ptr,
            usize::from_halves(num_rows, num_cols),
        );
        unsafe { &mut *(slice as *mut Self) }
    }

    pub fn from_slice(data: &[T], num_rows: usize, num_cols: usize) -> &Self {
        assert!(data.len() >= num_rows * num_cols);
        unsafe {
            Self::from_raw_parts(
                data.as_ptr(),
                num_rows.try_into().unwrap(),
                num_cols.try_into().unwrap(),
            )
        }
    }

    pub fn from_slice_mut(
        data: &mut [T],
        num_rows: usize,
        num_cols: usize,
    ) -> &mut Self {
        assert!(data.len() >= num_rows * num_cols);
        unsafe {
            Self::from_raw_parts_mut(
                data.as_mut_ptr(),
                num_rows.try_into().unwrap(),
                num_cols.try_into().unwrap(),
            )
        }
    }
}
