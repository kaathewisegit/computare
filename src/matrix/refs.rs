use core::{marker::PhantomData, ops::Deref};

use super::Matrix;

pub struct MatrixRef<'a, T> {
    ptr: *const T,
    num_rows: usize,
    num_cols: usize,
    marker: PhantomData<&'a T>,
}

impl<T> Matrix<T> for MatrixRef<'_, T> {
    unsafe fn at_u(&self, row: usize, col: usize) -> &T {
        let idx = self.index_of(row, col);
        unsafe { &*self.ptr.add(idx) }
    }

    fn num_rows(&self) -> usize {
        self.num_rows
    }

    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn row_stride(&self) -> usize {
        self.num_cols
    }
}

impl<T> Clone for MatrixRef<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for MatrixRef<'_, T> {}

impl<'a, T> MatrixRef<'a, T> {
    pub unsafe fn from_raw_parts(
        ptr: *const T,
        num_rows: usize,
        num_cols: usize,
    ) -> Self {
        Self {
            ptr,
            num_rows,
            num_cols,
            marker: PhantomData,
        }
    }

    pub unsafe fn u_new(
        data: &'a [T],
        num_rows: usize,
        num_cols: usize,
    ) -> Self {
        unsafe { Self::from_raw_parts(data.as_ptr(), num_rows, num_cols) }
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        self.num_cols * row + col
    }
}

pub struct MatrixRefMut<'a, T> {
    ptr: *mut T,
    num_rows: usize,
    num_cols: usize,
    marker: PhantomData<&'a mut T>,
}

impl<T> Matrix<T> for MatrixRefMut<'_, T> {
    unsafe fn at_u(&self, row: usize, col: usize) -> &T {
        let idx = self.index_of(row, col);
        unsafe { &*self.ptr.add(idx) }
    }

    fn num_rows(&self) -> usize {
        self.num_rows
    }

    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn row_stride(&self) -> usize {
        self.num_cols
    }
}

impl<'a, T> Deref for MatrixRefMut<'a, T> {
    type Target = MatrixRef<'a, T>;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `MatrixRef` has the same layout as `MatrixRefMut`, except for
        // `ptr` and `marker`.  `*const T` has the same layout as `*mut T`.  And
        // the marker has mutable and immutable references, but both of those
        // are covariant.
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl<'a, T> MatrixRefMut<'a, T> {
    pub unsafe fn from_raw_parts(
        ptr: *mut T,
        num_rows: usize,
        num_cols: usize,
    ) -> Self {
        Self {
            ptr,
            num_rows,
            num_cols,
            marker: PhantomData,
        }
    }

    pub unsafe fn u_new(
        data: &'a mut [T],
        num_rows: usize,
        num_cols: usize,
    ) -> Self {
        unsafe { Self::from_raw_parts(data.as_mut_ptr(), num_rows, num_cols) }
    }
}
