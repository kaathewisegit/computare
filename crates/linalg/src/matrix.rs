use core::ptr;

use crate::{StridedVectorRef, Vector};

pub trait Matrix {
    type Item;
    type Row: Vector<Item = Self::Item> + ?Sized;
    type Column: Vector<Item = Self::Item> + ?Sized;

    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn row_stride(&self) -> usize;

    unsafe fn at_u(&self, row: usize, col: usize) -> &Self::Item;

    fn at(&self, row: usize, col: usize) -> &Self::Item {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_u(row, col) }
    }

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut Self::Item;

    fn at_mut(&mut self, row: usize, col: usize) -> &mut Self::Item {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_mut_u(row, col) }
    }

    fn is_square(&self) -> bool {
        self.num_rows() == self.num_cols()
    }

    unsafe fn row_u(&self, index: usize) -> &Self::Row;

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut Self::Row;

    unsafe fn col_u(&self, index: usize) -> &Self::Column;

    unsafe fn col_mut_u(&mut self, index: usize) -> &mut Self::Column;

    /// # Safety
    ///
    /// - `a, b < self.num_rows()`
    unsafe fn swap_rows_u(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        for i in 0..self.num_cols() {
            unsafe { ptr::swap(self.at_mut_u(a, i), self.at_mut_u(b, i)) };
        }
    }

    fn for_each(&self, f: impl FnMut(&Self::Item));

    fn for_each_mut(&mut self, f: impl FnMut(&mut Self::Item));
}

impl<T, const N: usize, const M: usize> Matrix for [[T; M]; N] {
    type Item = T;
    type Row = [T; M];
    type Column = StridedVectorRef<T>;

    fn num_rows(&self) -> usize {
        N
    }

    fn num_cols(&self) -> usize {
        M
    }

    fn row_stride(&self) -> usize {
        M
    }

    unsafe fn at_u(&self, row: usize, col: usize) -> &T {
        unsafe { self.get_unchecked(row).get_unchecked(col) }
    }

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(row).get_unchecked_mut(col) }
    }

    unsafe fn row_u(&self, index: usize) -> &[T; M] {
        unsafe { self.get_unchecked(index) }
    }

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut [T; M] {
        unsafe { self.get_unchecked_mut(index) }
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

    fn for_each(&self, f: impl FnMut(&T)) {
        self.as_flattened().iter().for_each(f)
    }

    fn for_each_mut(&mut self, f: impl FnMut(&mut T)) {
        self.as_flattened_mut().iter_mut().for_each(f)
    }
}
