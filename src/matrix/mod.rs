use core::ptr;

mod refs;

pub use refs::{MatrixRef, MatrixRefMut};

pub trait Matrix<T> {
    unsafe fn at_u(&self, row: usize, col: usize) -> &T;

    fn at(&self, row: usize, col: usize) -> &T {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_u(row, col) }
    }

    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn row_stride(&self) -> usize;
}

pub trait MatrixMut<T>: Matrix<T> {
    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T;

    fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < self.num_rows() && col < self.num_rows());
        unsafe { self.at_mut_u(row, col) }
    }
}

impl<T, const N: usize, const M: usize> Matrix<T> for [[T; M]; N] {
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
}

impl<T, const N: usize, const M: usize> MatrixMut<T> for [[T; M]; N] {
    unsafe fn at_mut_u(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(row).get_unchecked_mut(col) }
    }
}

pub unsafe fn swap_rows_u<T, M>(m: &mut M, a: usize, b: usize)
where
    M: MatrixMut<T> + ?Sized,
{
    if a == b {
        return;
    }

    let a_ptr = unsafe { m.at_mut_u(a, 0) } as *mut T;
    let b_ptr = unsafe { m.at_mut_u(b, 0) } as *mut T;

    unsafe { ptr::swap_nonoverlapping(a_ptr, b_ptr, m.num_rows()) }
}
