use alloc::alloc::{Layout, alloc, alloc_zeroed, dealloc, handle_alloc_error};
use core::{
    cmp,
    ptr::{
        NonNull, copy_nonoverlapping, slice_from_raw_parts,
        slice_from_raw_parts_mut,
    },
};

/// A contiguous array allocation for `T`
///
/// Welcome to the undefined behavior galore.
pub struct RawBuffer<T, const ALIGN: usize = 0> {
    ptr: NonNull<T>,
}

// SAFETY: it's only an allocation
unsafe impl<T, const ALIGN: usize> Send for RawBuffer<T, ALIGN> where T: Send {}
// SAFETY: same as above
unsafe impl<T, const ALIGN: usize> Sync for RawBuffer<T, ALIGN> where T: Sync {}

impl<T, const ALIGN: usize> RawBuffer<T, ALIGN> {
    pub const ALIGNMENT: usize =
        { if ALIGN == 0 { align_of::<T>() } else { ALIGN } };

    /// ```compile_fail
    /// buffer::Buffer::<u8, 3>::new(10);
    /// ```
    const _CHECK_ALIGN_POW2: () = assert!(
        ALIGN == 0 || ALIGN.is_power_of_two(),
        "ALIGN must be a power of two"
    );

    /// ```compile_fail
    /// buffer::Buffer::<u64, 2>::new(10);
    /// ```
    const _CHECK_ALIGN_SIZE: () = assert!(
        ALIGN == 0 || ALIGN >= align_of::<T>(),
        "ALIGN be equal to or bigger than `T`'s required alignment"
    );

    /// ```compile_fail
    /// buffer::Buffer::<(), 3>::new(10);
    /// ```
    const _NON_ZST: () = assert!(size_of::<T>() != 0, "T must not be a ZST");

    fn layout(capacity: usize) -> Layout {
        assert_ne!(capacity, 0);
        Layout::from_size_align(capacity * size_of::<T>(), Self::ALIGNMENT)
            .expect("`capacity` too big")
    }

    pub fn dangling() -> Self {
        #[expect(unused)]
        {
            Self::_CHECK_ALIGN_POW2;
            Self::_CHECK_ALIGN_SIZE;
            Self::_NON_ZST;
        }
        Self {
            ptr: NonNull::dangling(),
        }
    }

    /// Allocate a new uninitialized buffer of size `capacity`.
    pub fn uninit(capacity: usize) -> Self {
        #[expect(unused)]
        {
            Self::_CHECK_ALIGN_POW2;
            Self::_CHECK_ALIGN_SIZE;
            Self::_NON_ZST;
        }

        if capacity == 0 {
            return Self::dangling();
        }

        let layout = Self::layout(capacity);

        // SAFETY: we've checked above that capacity/size isn't 0 and
        // there's a const assertion that size_of::<T> isn't 0.
        let ptr = unsafe { alloc(layout) as *mut T };
        let Some(ptr) = NonNull::new(ptr) else {
            handle_alloc_error(layout);
        };

        Self { ptr }
    }

    /// Allocates a new buffer of size `capacity` with all bits set to 0
    pub fn zeroed(capacity: usize) -> Self {
        #[expect(unused)]
        {
            Self::_CHECK_ALIGN_POW2;
            Self::_CHECK_ALIGN_SIZE;
            Self::_NON_ZST;
        }

        if capacity == 0 {
            return Self::dangling();
        }

        let layout = Self::layout(capacity);

        // SAFETY: we've checked above that capacity/size isn't 0 and
        // there's a const assertion that size_of::<T> isn't 0.
        let ptr = unsafe { alloc_zeroed(layout) as *mut T };
        let Some(ptr) = NonNull::new(ptr) else {
            handle_alloc_error(layout);
        };

        Self { ptr }
    }

    /// Allocates a buffer of different length and copies elements into it
    ///
    /// Note that if `new_capacity < old_capacity` the buffer will shrink and
    /// the elements which don't fit into the new buffer will be forgotten.  If
    /// `new_capacity > old_capacity` the remaining space will be filled by
    /// poisoned values, which must not be read or included in slices.
    ///
    /// # Safety
    ///
    /// - `old_capacity` must be the current capacity of the buffer.
    /// - `new_capacity` must not be zero.
    pub unsafe fn reallocate(
        &mut self,
        old_capacity: usize,
        new_capacity: usize,
    ) {
        // SAFETY: `new_capacity` is not zero per the function invariant
        let new_buf = Self::uninit(new_capacity);
        let count = cmp::min(old_capacity, new_capacity);
        // SAFETY: `count` fits inside both buffers, the old buffer is
        // discarded right after.
        if old_capacity != 0 {
            unsafe {
                copy_nonoverlapping::<T>(
                    self.ptr.as_ptr(),
                    new_buf.ptr.as_ptr(),
                    count,
                );
                self.deallocate(old_capacity);
            }
        }
        *self = new_buf;
    }

    /// Zeroed version of `reallocate`
    ///
    /// # Safety
    ///
    /// See [`Self::reallocate`].
    pub unsafe fn reallocate_zeroed(
        &mut self,
        old_capacity: usize,
        new_capacity: usize,
    ) {
        // SAFETY: `new_capacity` is not zero per the function invariant
        let new_buf = Self::zeroed(new_capacity);
        let count = cmp::min(old_capacity, new_capacity);
        // SAFETY: `count` fits inside both buffers, the old buffer is
        // discarded right after.
        if old_capacity != 0 {
            unsafe {
                copy_nonoverlapping::<T>(
                    self.ptr.as_ptr(),
                    new_buf.ptr.as_ptr(),
                    count,
                );
                self.deallocate(old_capacity);
            }
        }
        *self = new_buf;
    }

    pub fn from_slice(slice: &[T]) -> Self {
        let len = slice.len();
        if len == 0 {
            return Self::dangling();
        }
        // SAFETY: we'll overwrite them
        let out = Self::uninit(len);
        // SAFETY: both `slice` and `out` have the length of `len`,
        // `out` is writeable
        unsafe {
            copy_nonoverlapping::<T>(slice.as_ptr(), out.ptr().as_ptr(), len)
        }
        out
    }

    /// # Safety
    ///
    /// - `index * size_of::<T>()` must not overflow `isize`.
    /// - `index` must be less than the buffer capacity.
    ///
    /// Additionally, the resulting pointer is only valid if `idx` is
    /// within the buffer's capacity.
    pub unsafe fn get(&self, index: usize) -> *const T {
        // SAFETY: function's unsafe invariant
        unsafe { self.ptr.add(index) }.as_ptr()
    }

    /// # Safety
    ///
    /// See [`Self::get`].
    pub unsafe fn get_mut(&self, index: usize) -> *mut T {
        // SAFETY: same as `get`
        (unsafe { self.get(index) }) as *mut T
    }

    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }

    pub fn ptr_raw(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// # Safety
    ///
    /// For the resulting `[T]` to be usable, the following must be satisfied:
    ///
    /// - `offset + len` must be less or equal to the buffer's capacity.
    /// - All elements in the slice must be initialized.
    pub unsafe fn get_raw_slice_mut(
        &mut self,
        offset: usize,
        len: usize,
    ) -> *mut [T] {
        // SAFETY: (from `slice::from_raw_parts_mut`)
        // - `ptr` is non-null, valid, and points to an allocation
        //   `capacity * size_of::<T>()` large.
        // - The data is aligned
        // - The reference is unique
        // - Total size is less than `isize::MAX`, checked by
        //   `Layout::from_size_align`
        slice_from_raw_parts_mut(unsafe { self.ptr_raw().add(offset) }, len)
    }

    /// # Safety
    ///
    /// See [`Self::as_raw_slice_mut`].
    pub unsafe fn get_raw_slice(
        &self,
        offset: usize,
        len: usize,
    ) -> *const [T] {
        // SAFETY: same as `as_raw_slice_mut`
        slice_from_raw_parts(unsafe { self.ptr_raw().add(offset) }, len)
    }

    /// Deallocate the buffer
    ///
    /// Note that the caller is responsible for dropping the elements of the
    /// buffer if they require that.
    ///
    /// # Safety
    ///
    /// The `capacity` must be correct.  The slice **must not be used** after
    /// this method is called.
    pub unsafe fn deallocate(&mut self, capacity: usize) {
        if capacity == 0 {
            return;
        }
        let layout = Self::layout(capacity);
        // SAFETY: capacity (and thus layout) should be valid by the
        // function invariant.
        unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout) };
    }
}
