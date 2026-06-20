// Contrary to what the lint says, I find the arithmetic to be clearer with an
// explicit +1
#![expect(clippy::int_plus_one)]

use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use super::Vector;
use crate::packing::Packed;

/// A non-contiguous vector reference
///
/// See [`MatrixRef`] for details on how this type is implemented.
#[repr(transparent)]
pub struct StridedVectorRef<T>([T]);

impl<T> Vector<T> for StridedVectorRef<T> {
    type Slice = StridedVectorRef<T>;

    fn length(&self) -> usize {
        self.0.len().lower()
    }

    fn stride(&self) -> usize {
        self.0.len().upper()
    }

    unsafe fn at_u(&self, index: usize) -> &T {
        unsafe { &*self.as_ptr().add(self.stride() * index) }
    }

    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T {
        unsafe { &mut *self.as_mut_ptr().add(self.stride() * index) }
    }

    unsafe fn slice_u(&self, start: usize, end: usize) -> &Self::Slice {
        unsafe {
            Self::from_raw_parts(
                self.as_ptr().add(self.stride() * start),
                (end - start) as u32,
                self.stride() as u32,
            )
        }
    }

    unsafe fn slice_mut_u(
        &mut self,
        start: usize,
        end: usize,
    ) -> &mut Self::Slice {
        unsafe {
            Self::from_raw_parts_mut(
                self.as_mut_ptr().add(self.stride() * start),
                (end - start) as u32,
                self.stride() as u32,
            )
        }
    }
}

impl<T> StridedVectorRef<T> {
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    #[cfg(target_pointer_width = "64")]
    pub unsafe fn from_raw_parts<'a>(
        ptr: *const T,
        len: u32,
        stride: u32,
    ) -> &'a Self {
        let slice = slice_from_raw_parts(ptr, usize::from_halves(len, stride));
        unsafe { &*(slice as *const Self) }
    }

    #[cfg(target_pointer_width = "64")]
    pub unsafe fn from_raw_parts_mut<'a>(
        ptr: *mut T,
        len: u32,
        stride: u32,
    ) -> &'a mut Self {
        let slice =
            slice_from_raw_parts_mut(ptr, usize::from_halves(len, stride));
        unsafe { &mut *(slice as *mut Self) }
    }

    pub fn from_slice(data: &[T], len: usize, stride: usize) -> &Self {
        assert!(len == 0 || data.len() >= (len - 1) * stride + 1);
        // SAFETY: `data` is valid, and the new slice is either empty or is long
        // enough to fit `len` elements and `len - 1` strides inbetween.
        unsafe {
            Self::from_raw_parts(
                data.as_ptr(),
                len.try_into().unwrap(),
                stride.try_into().unwrap(),
            )
        }
    }

    pub fn from_slice_mut(
        data: &mut [T],
        len: usize,
        stride: usize,
    ) -> &mut Self {
        assert!(len == 0 || data.len() >= (len - 1) * stride + 1);
        // SAFETY: see `from_slice`
        unsafe {
            Self::from_raw_parts_mut(
                data.as_mut_ptr(),
                len.try_into().unwrap(),
                stride.try_into().unwrap(),
            )
        }
    }
}
