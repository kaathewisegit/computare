mod refs;

use core::ptr;

pub use refs::MatrixRef;

use crate::vector::Vector;

pub trait Matrix<T> {
    type Row: Vector<T> + ?Sized;

    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn row_stride(&self) -> usize;

    unsafe fn at_u(&self, row: usize, col: usize) -> &T;

    fn at(&self, row: usize, col: usize) -> &T {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_u(row, col) }
    }

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T;

    fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_mut_u(row, col) }
    }

    fn is_square(&self) -> bool {
        self.num_rows() == self.num_cols()
    }

    unsafe fn row_u(&self, index: usize) -> &Self::Row;

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut Self::Row;

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

    fn for_each(&self, f: impl FnMut(&T));

    fn for_each_mut(&mut self, f: impl FnMut(&mut T));
}

impl<T, const N: usize, const M: usize> Matrix<T> for [[T; M]; N] {
    type Row = [T; M];

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

    fn for_each(&self, f: impl FnMut(&T)) {
        self.as_flattened().iter().for_each(f)
    }

    fn for_each_mut(&mut self, f: impl FnMut(&mut T)) {
        self.as_flattened_mut().iter_mut().for_each(f)
    }
}
