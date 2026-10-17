use core::{
    ops::{Deref, DerefMut},
    ptr,
};

use super::MatrixRef;
use computare_buffer::RawBuffer;

/// Owned row-major matrix
pub struct RowMatrixBuf<T> {
    values: RawBuffer<T>,
    num_rows: u32,
    num_cols: u32,
}

impl<T> RowMatrixBuf<T> {
    pub fn len(&self) -> usize {
        self.num_rows() * self.num_cols()
    }

    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }

    pub fn as_slice_ptr(&self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.as_ptr(), self.len())
    }

    pub fn from_slice(slice: &[T], num_rows: usize, num_cols: usize) -> Self {
        if num_rows == 0 || num_cols == 0 {
            return Self::empty();
        }

        Self {
            values: RawBuffer::from_slice(slice),
            num_rows: u32::try_from(num_rows).unwrap(),
            num_cols: u32::try_from(num_cols).unwrap(),
        }
    }

    pub fn empty() -> Self {
        Self {
            values: RawBuffer::dangling(),
            num_rows: 0,
            num_cols: 0,
        }
    }

    pub fn num_rows(&self) -> usize {
        usize::try_from(self.num_rows).unwrap()
    }

    pub fn num_cols(&self) -> usize {
        usize::try_from(self.num_cols).unwrap()
    }

    pub fn num_elements(&self) -> usize {
        self.num_rows() * self.num_cols()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.values.ptr_raw()
    }
}

impl<T> Drop for RowMatrixBuf<T> {
    fn drop(&mut self) {
        unsafe { self.values.deallocate(self.len()) }
    }
}

// SAFETY: `RowMatrixBuf` owns its items.
unsafe impl<T: Send> Send for RowMatrixBuf<T> {}
// SAFETY: `RowMatrixBuf` owns its items.
unsafe impl<T: Sync> Sync for RowMatrixBuf<T> {}

impl<T> Deref for RowMatrixBuf<T> {
    type Target = MatrixRef<T>;

    fn deref(&self) -> &MatrixRef<T> {
        unsafe {
            MatrixRef::from_raw_parts(
                self.as_ptr(),
                self.num_rows,
                self.num_cols,
            )
        }
    }
}

impl<T> DerefMut for RowMatrixBuf<T> {
    fn deref_mut(&mut self) -> &mut MatrixRef<T> {
        unsafe {
            MatrixRef::from_raw_parts_mut(
                self.as_ptr(),
                self.num_rows,
                self.num_cols,
            )
        }
    }
}
