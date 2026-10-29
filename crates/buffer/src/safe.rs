use bytemuck::Zeroable;

use core::{
    fmt,
    ops::{
        Bound, Deref, DerefMut, Index, IndexMut, Range, RangeBounds, RangeTo,
    },
};

use crate::RawBuffer;

/// A heap-allocated, fixed-size array of initialized values
///
/// This is the safe counterpart to [`RawBuffer`].  Unlike the raw buffer, every
/// element in the allocation is guaranteed to be initialized, and ordinary safe
/// methods can be used to access the data.
pub struct Buffer<T: Copy, const ALIGN: usize = 0> {
    raw: RawBuffer<T, ALIGN>,
    len: usize,
}

impl<T: Copy, const ALIGN: usize> Buffer<T, ALIGN> {
    /// The number of elements in the buffer.
    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn check_index(&self, index: usize) {
        assert!(
            index < self.len,
            "index {index} is out of bounds, this `Buffer` has length {}",
            self.len
        );
    }

    fn to_range(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Unbounded => 0,
            _ => unreachable!(),
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.saturating_add(1),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len,
        };

        assert!(
            start <= end && end <= self.len,
            "range start {start} must be <= range end {end}, and end must be \
             <= this `Buffer`'s length {}",
            self.len
        );

        (start, end - start)
    }

    /// # Safety
    ///
    /// `index` must be less than `self.len()`.
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        if cfg!(debug_assertions) {
            self.check_index(index);
        }

        // SAFETY: the index is within the fully initialized allocation.
        unsafe { &*self.raw.get(index) }
    }

    /// # Safety
    ///
    /// `index` must be less than `self.len()`.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        if cfg!(debug_assertions) {
            self.check_index(index);
        }

        // SAFETY: the index is within the fully initialized allocation, and
        // `self` is exclusively borrowed.
        unsafe { &mut *self.raw.get_mut(index) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SAFETY: the index was checked above.
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            // SAFETY: the index was checked above.
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: all elements are initialized and `len` is the allocation's
        // exact size.
        unsafe { &*self.raw.get_raw_slice(0, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: all elements are initialized, `len` is the allocation's
        // exact size, and `self` is exclusively borrowed.
        unsafe { &mut *self.raw.get_raw_slice_mut(0, self.len) }
    }

    pub fn empty() -> Self {
        Self {
            raw: RawBuffer::dangling(),
            len: 0,
        }
    }

    pub fn zeroed(len: usize) -> Self
    where
        T: Zeroable,
    {
        Self {
            raw: RawBuffer::zeroed(len),
            len,
        }
    }

    pub fn repeat(value: T, len: usize) -> Self {
        Self {
            raw: RawBuffer::repeat(value, len),
            len,
        }
    }

    pub fn from_slice(values: &[T]) -> Self {
        Self {
            raw: RawBuffer::from_slice(values),
            len: values.len(),
        }
    }
}

impl<T: Copy, const ALIGN: usize> Clone for Buffer<T, ALIGN> {
    fn clone(&self) -> Self {
        if self.len == 0 {
            return Self::empty();
        }

        // SAFETY: `len` is the current capacity.
        let raw = unsafe { self.raw.duplicate(self.len) };
        Self { raw, len: self.len }
    }
}

impl<T: Copy, const ALIGN: usize> Drop for Buffer<T, ALIGN> {
    fn drop(&mut self) {
        // SAFETY: `len` is the buffer's exact allocation capacity.
        unsafe { self.raw.deallocate(self.len) };
    }
}

impl<T: Copy, const ALIGN: usize> From<&[T]> for Buffer<T, ALIGN> {
    fn from(values: &[T]) -> Self {
        Self::from_slice(values)
    }
}

impl<T: Copy + fmt::Debug, const ALIGN: usize> fmt::Debug for Buffer<T, ALIGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T: Copy, const ALIGN: usize> Deref for Buffer<T, ALIGN> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: Copy, const ALIGN: usize> DerefMut for Buffer<T, ALIGN> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: Copy, const ALIGN: usize> AsRef<[T]> for Buffer<T, ALIGN> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: Copy, const ALIGN: usize> AsMut<[T]> for Buffer<T, ALIGN> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: Copy, const ALIGN: usize> Index<usize> for Buffer<T, ALIGN> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.check_index(index);

        // SAFETY: the index was checked above.
        unsafe { self.get_unchecked(index) }
    }
}

impl<T: Copy, const ALIGN: usize> IndexMut<usize> for Buffer<T, ALIGN> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.check_index(index);

        // SAFETY: the index was checked above.
        unsafe { self.get_unchecked_mut(index) }
    }
}

impl<T: Copy, const ALIGN: usize> Index<Range<usize>> for Buffer<T, ALIGN> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &[T] {
        let (start, len) = self.to_range(index);

        // SAFETY: the range was checked above.
        unsafe { &*self.raw.get_raw_slice(start, len) }
    }
}

impl<T: Copy, const ALIGN: usize> Index<RangeTo<usize>> for Buffer<T, ALIGN> {
    type Output = [T];

    fn index(&self, index: RangeTo<usize>) -> &[T] {
        let (start, len) = self.to_range(index);

        // SAFETY: the range was checked above.
        unsafe { &*self.raw.get_raw_slice(start, len) }
    }
}

impl<T: Copy, const ALIGN: usize> IndexMut<Range<usize>> for Buffer<T, ALIGN> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        let (start, len) = self.to_range(index);

        // SAFETY: the range was checked above.
        unsafe { &mut *self.raw.get_raw_slice_mut(start, len) }
    }
}

impl<T: Copy, const ALIGN: usize> IndexMut<RangeTo<usize>>
    for Buffer<T, ALIGN>
{
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        let (start, len) = self.to_range(index);

        // SAFETY: the range was checked above.
        unsafe { &mut *self.raw.get_raw_slice_mut(start, len) }
    }
}
