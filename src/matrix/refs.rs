use core::{
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use super::Matrix;

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

    #[cfg(target_pointer_width = "64")]
    fn num_rows(&self) -> usize {
        self.0.len() >> 32
    }

    #[cfg(target_pointer_width = "64")]
    fn num_cols(&self) -> usize {
        self.0.len() as u32 as usize
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
}

impl<T> MatrixRef<T> {
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        self.num_cols() * row + col
    }
}

impl<T> MatrixRef<T> {
    pub unsafe fn from_raw_parts<'a>(
        ptr: *const T,
        num_rows: u32,
        num_cols: u32,
    ) -> &'a Self {
        #[cfg(target_pointer_width = "64")]
        let metadata = ((num_rows as usize) << 32) + num_cols as usize;
        let slice = slice_from_raw_parts(ptr, metadata);
        unsafe { &*(slice as *const Self) }
    }

    pub unsafe fn from_raw_parts_mut<'a>(
        ptr: *mut T,
        num_rows: u32,
        num_cols: u32,
    ) -> &'a mut Self {
        #[cfg(target_pointer_width = "64")]
        let metadata = ((num_rows as usize) << 32) + num_cols as usize;
        let slice = slice_from_raw_parts_mut(ptr, metadata);
        unsafe { &mut *(slice as *mut Self) }
    }
}
