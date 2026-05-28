use super::Vector;
use crate::packing::Packed;

/// A non-contiguous vector reference
///
/// See [`MatrixRef`] for details on how this type is implemented.
#[repr(transparent)]
pub struct StridedVectorRef<T>([T]);

impl<T> Vector<T> for StridedVectorRef<T> {
    fn length(&self) -> usize {
        self.0.len().lower()
    }

    fn stride(&self) -> usize {
        self.0.len().upper()
    }

    unsafe fn at_u(&self, index: usize) -> &T {
        unsafe { &*self.as_ptr().add(index) }
    }

    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T {
        unsafe { &mut *self.as_mut_ptr().add(index) }
    }
}

impl<T> StridedVectorRef<T> {
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }
}
