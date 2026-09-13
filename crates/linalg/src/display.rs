use core::fmt::{Debug, Display, Formatter, Result};
use std::fmt::Write;

use crate::{Matrix, Vector};

pub struct DisplayVector<'a, V: ?Sized>(pub &'a V);

struct VectorHelper<'a, V: ?Sized>(&'a V);

impl<V: Vector + ?Sized> Debug for VectorHelper<'_, V>
where
    V::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut list = f.debug_list();
        for i in 0..self.0.length() {
            list.entry(self.0.at(i));
        }
        list.finish()
    }
}

impl<V: Vector + ?Sized> Debug for DisplayVector<'_, V>
where
    V::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Vector")
            .field("length", &self.0.length())
            .field("stride", &self.0.stride())
            .field("values", &VectorHelper(self.0))
            .finish()
    }
}

impl<V: Vector + ?Sized> Display for DisplayVector<'_, V>
where
    V::Item: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("[")?;
        for i in 0..self.0.length() {
            if i != 0 {
                f.write_str(", ")?;
            }
            write!(f, "{}", self.0.at(i))?;
        }
        f.write_str("]")
    }
}

pub struct DisplayMatrix<'a, M: ?Sized>(pub &'a M);

struct MatrixHelper<'a, M: ?Sized>(&'a M);

impl<M: Matrix + ?Sized> Debug for MatrixHelper<'_, M>
where
    M::Item: Debug,
    M::Row: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut list = f.debug_list();
        for i in 0..self.0.num_rows() {
            let row = unsafe { self.0.row_u(i) };
            list.entry(&row);
        }
        list.finish()
    }
}

impl<M: Matrix + ?Sized> Debug for DisplayMatrix<'_, M>
where
    M::Item: Debug,
    M::Row: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Vector")
            .field("num_rows", &self.0.num_rows())
            .field("num_cols", &self.0.num_cols())
            .field("values", &MatrixHelper(self.0))
            .finish()
    }
}

impl<M: Matrix + ?Sized> Display for DisplayMatrix<'_, M>
where
    M::Item: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('[')?;
        for i in 0..self.0.num_cols() {
            if i != 0 {
                f.write_char(',')?;
                if f.alternate() {
                    f.write_char('\n')?;
                }
                f.write_char(' ')?;
            }

            f.write_str("[")?;
            for j in 0..self.0.num_rows() {
                if j != 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{}", self.0.at(i, j))?;
            }
            f.write_char(']')?;
        }
        f.write_char(']')
    }
}
