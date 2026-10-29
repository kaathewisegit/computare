#![no_std]

extern crate alloc;

mod raw;
mod safe;
mod slice;

pub use raw::RawBuffer;
pub use safe::Buffer;
pub use slice::SliceBuffer;
