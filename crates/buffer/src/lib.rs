#![no_std]

extern crate alloc;

mod raw;
mod slice;

pub use raw::RawBuffer;
pub use slice::SliceBuffer;
