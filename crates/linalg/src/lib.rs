#![allow(clippy::missing_safety_doc)]

mod display;
pub(crate) mod int_utils;
mod matrix;
mod matrix_refs;
pub mod ops;
pub(crate) mod packing;
mod vector;
mod vector_strided;

pub use display::{DisplayMatrix, DisplayVector};
pub use matrix::Matrix;
pub use matrix_refs::MatrixRef;
pub use vector::Vector;
pub use vector_strided::StridedVectorRef;
