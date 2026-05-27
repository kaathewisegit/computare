mod refs;

use core::ptr;

pub use refs::{MatrixRef, MatrixRefMut};

use crate::vector::{Vector, VectorMut};

pub trait Matrix<T> {
    type Row: Vector<T> + ?Sized;

    unsafe fn at_u(&self, row: usize, col: usize) -> &T;

    fn at(&self, row: usize, col: usize) -> &T {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_u(row, col) }
    }

    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn row_stride(&self) -> usize;

    fn is_square(&self) -> bool {
        self.num_rows() == self.num_cols()
    }

    unsafe fn row_u(&self, index: usize) -> &Self::Row;
}

pub trait MatrixMut<T>: Matrix<T> {
    type RowMut: VectorMut<T> + ?Sized;

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T;

    fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_mut_u(row, col) }
    }

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut Self::RowMut;

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
}

impl<T, const N: usize, const M: usize> Matrix<T> for [[T; M]; N] {
    type Row = [T; M];

    unsafe fn at_u(&self, row: usize, col: usize) -> &T {
        unsafe { self.get_unchecked(row).get_unchecked(col) }
    }

    fn num_rows(&self) -> usize {
        N
    }

    fn num_cols(&self) -> usize {
        M
    }

    fn row_stride(&self) -> usize {
        M
    }

    unsafe fn row_u(&self, index: usize) -> &Self::Row {
        unsafe { self.get_unchecked(index) }
    }
}

impl<T, const N: usize, const M: usize> MatrixMut<T> for [[T; M]; N] {
    type RowMut = [T; M];

    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(row).get_unchecked_mut(col) }
    }

    unsafe fn row_mut_u(&mut self, index: usize) -> &mut [T; M] {
        unsafe { self.get_unchecked_mut(index) }
    }
}
