use core::{
    marker::PhantomData,
    ptr::{self, slice_from_raw_parts, slice_from_raw_parts_mut},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use super::Matrix;
use crate::{StridedVectorRef, packing::Packed};

// This is a hack I came up with after several iterations.  The issue is that in
// Rust only `&` and `&mut` are first class references.  The previous design
// used two objects, `MatrixRef(*const T, usize, usize)` and `MatrixRefMut(*mut
// T, usize, usize)`, but it was suboptimal.  `MatrixRefMut` had to be
// reborrowed and I needed two traits (`Matrix` and `MatrixMut`).  Furthermore,
// in those traits I had to either take `self` by value complicating working
// with other objects or taking it by reference, meaning it'd be a double
// pointer.
//
// The original implementation used `[T]` with a packed length field, which was
// nominally unsound, because it was constructing an invalid slice.  Now, I
// never used it, so in practice the unsoundness never showed itself, but it was
// still a suboptimal solution (and it tripped up Miri).  I stumbled upon a
// better solution with `bitvec`'s `BitSlice`.  `[()]` is also unsized with two
// words of metadata, but because it's a ZST the pointer and length can be
// arbitrary.  And it works with Miri under tree borrows.
#[repr(transparent)]
pub struct MatrixRef<T> {
    marker: PhantomData<T>,
    ptr: [()],
}

impl<T> Matrix for MatrixRef<T> {
    type Item = T;
    type Row = [T];
    type Column = StridedVectorRef<T>;

    #[cfg(target_pointer_width = "64")]
    fn num_rows(&self) -> usize {
        self.ptr.len().lower()
    }

    #[cfg(target_pointer_width = "64")]
    fn num_cols(&self) -> usize {
        self.ptr.len().upper()
    }

    fn stride_col(&self) -> usize {
        1
    }

    fn stride_row(&self) -> usize {
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
                self.stride_row() as u32,
            )
        }
    }

    unsafe fn col_mut_u(&mut self, index: usize) -> &mut StridedVectorRef<T> {
        unsafe {
            StridedVectorRef::from_raw_parts_mut(
                Matrix::at_mut_u(self, 0, index) as *mut T,
                self.num_rows() as u32,
                self.stride_row() as u32,
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
        self.ptr.as_ptr() as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_mut_ptr() as *mut T
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
        assert_eq!(data.len(), num_rows * num_cols);
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
        assert_eq!(data.len(), num_rows * num_cols);
        unsafe {
            Self::from_raw_parts_mut(
                data.as_mut_ptr(),
                num_rows.try_into().unwrap(),
                num_cols.try_into().unwrap(),
            )
        }
    }
}

/// Column-major matrix reference
#[repr(transparent)]
pub struct ColMatrixRef<T> {
    marker: PhantomData<T>,
    ptr: [()],
}

impl<T> Matrix for ColMatrixRef<T> {
    type Item = T;
    type Row = StridedVectorRef<T>;
    type Column = [T];

    #[cfg(target_pointer_width = "64")]
    fn num_rows(&self) -> usize {
        self.ptr.len().lower()
    }

    #[cfg(target_pointer_width = "64")]
    fn num_cols(&self) -> usize {
        self.ptr.len().upper()
    }

    fn stride_col(&self) -> usize {
        self.num_rows()
    }

    fn stride_row(&self) -> usize {
        1
    }

    unsafe fn at_u(&self, row: usize, col: usize) -> &T {
        let idx = self.index_of(row, col);
        unsafe { &*self.as_ptr().add(idx) }
    }

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.index_of(row, col);
        unsafe { &mut *self.as_mut_ptr().add(idx) }
    }

    unsafe fn row_u(&self, index: usize) -> &StridedVectorRef<T> {
        unsafe {
            StridedVectorRef::from_raw_parts(
                Matrix::at_u(self, index, 0) as *const T,
                self.num_cols() as u32,
                self.stride_col() as u32,
            )
        }
    }

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut StridedVectorRef<T> {
        unsafe {
            StridedVectorRef::from_raw_parts_mut(
                Matrix::at_mut_u(self, index, 0) as *mut T,
                self.num_cols() as u32,
                self.stride_col() as u32,
            )
        }
    }

    unsafe fn col_u(&self, index: usize) -> &[T] {
        unsafe {
            let ptr = self.as_ptr().add(index * self.num_rows());
            from_raw_parts(ptr, self.num_rows())
        }
    }

    unsafe fn col_mut_u(&mut self, index: usize) -> &mut [T] {
        unsafe {
            let ptr = self.as_mut_ptr().add(index * self.num_rows());
            from_raw_parts_mut(ptr, self.num_rows())
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

impl<T> ColMatrixRef<T> {
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr() as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_mut_ptr() as *mut T
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
        row + col * self.num_rows()
    }

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
        assert_eq!(data.len(), num_rows * num_cols);
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
        assert_eq!(data.len(), num_rows * num_cols);
        unsafe {
            Self::from_raw_parts_mut(
                data.as_mut_ptr(),
                num_rows.try_into().unwrap(),
                num_cols.try_into().unwrap(),
            )
        }
    }
}
