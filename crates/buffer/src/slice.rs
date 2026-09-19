use bytemuck::Zeroable;

use core::{
    mem,
    ops::{Index, IndexMut},
    ptr,
};

use crate::RawBuffer;

pub struct SliceBuffer<T, const ALIGN: usize = 0> {
    buffer: RawBuffer<T, ALIGN>,
    /// Size of each slice segment
    size: usize,
    /// Total number of slice segments
    len: usize,
}

impl<T, const ALIGN: usize> SliceBuffer<T, ALIGN> {
    /// Creates a buffer capable of holding `len` slices of `size`
    pub fn new(size: usize, len: usize) -> Self
    where
        T: Zeroable,
    {
        let capacity = size * len;
        assert_ne!(capacity, 0, "SliceBuffer size/length must not be zero");
        // SAFETY: `T` is `Zeroable`, so all of the elements are initialized.
        // `capacity` is checked to not be zero.
        let buffer = unsafe { RawBuffer::zeroed(capacity) };
        Self { buffer, size, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len != 0
    }

    fn check_index(&self, index: usize) {
        assert!(
            index < self.len,
            "Index {} out of bounds, this `SliceBuffer` only has {} slices",
            index,
            self.len
        );
    }

    fn raw_slice(&self, index: usize) -> (usize, usize) {
        let size = self.size;
        let offset = size * index;
        (offset, size)
    }

    /// # Safety
    ///
    /// `index` must be less than `self.len()`
    pub unsafe fn get_unchecked(&self, index: usize) -> &[T] {
        if cfg!(debug_assertions) {
            self.check_index(index)
        }
        let (offset, length) = self.raw_slice(index);
        // SAFETY: per the function invariant `range(index)` must be valid.  All
        // elements have been initialized at allocation.
        unsafe { &*self.buffer.get_raw_slice(offset, length) }
    }

    /// # Safety
    ///
    /// `index` must be less than `self.len()`
    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut [T] {
        if cfg!(debug_assertions) {
            self.check_index(index)
        }
        let (offset, length) = self.raw_slice(index);
        // SAFETY: per the function invariant `range(index)` must be valid.  All
        // elements have been initialized at allocation.
        unsafe { &mut *self.buffer.get_raw_slice_mut(offset, length) }
    }
}

impl<T, const ALIGN: usize> Drop for SliceBuffer<T, ALIGN> {
    fn drop(&mut self) {
        let capacity = self.len * self.size;
        if const { mem::needs_drop::<T>() } {
            // SAFETY: `[0, capacity)` covers the whole allocation exactly
            let all = unsafe { self.buffer.get_raw_slice_mut(0, capacity) };
            // SAFETY: see above.  We will not be touching these elements again
            unsafe { ptr::drop_in_place(all) }
        }
        // SAFETY: `capacity` is correct
        unsafe { self.buffer.deallocate(capacity) }
    }
}

impl<T, const ALIGN: usize> Index<usize> for SliceBuffer<T, ALIGN> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        self.check_index(index);
        // SAFETY: the index is checked above
        unsafe { self.get_unchecked(index) }
    }
}

impl<T, const ALIGN: usize> IndexMut<usize> for SliceBuffer<T, ALIGN> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        self.check_index(index);
        // SAFETY: the index is checked above
        unsafe { self.get_mut_unchecked(index) }
    }
}
