mod strided;

pub use strided::StridedVectorRef;

pub trait Vector<T> {
    type Slice: Vector<T> + ?Sized;

    fn length(&self) -> usize;

    fn stride(&self) -> usize;

    unsafe fn at_u(&self, index: usize) -> &T;

    fn at(&self, index: usize) -> &T {
        assert!(index < self.length());
        unsafe { self.at_u(index) }
    }

    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T;

    fn at_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.length());
        unsafe { self.at_mut_u(index) }
    }

    unsafe fn slice_u(&self, start: usize, end: usize) -> &Self::Slice;

    fn slice(&self, start: usize, end: usize) -> &Self::Slice {
        assert!(start < end);
        assert!(end < self.length());
        unsafe { self.slice_u(start, end) }
    }

    unsafe fn slice_mut_u(
        &mut self,
        start: usize,
        end: usize,
    ) -> &mut Self::Slice;

    fn slice_mut(&mut self, start: usize, end: usize) -> &mut Self::Slice {
        assert!(start < end);
        assert!(end < self.length());
        unsafe { self.slice_mut_u(start, end) }
    }
}

impl<T, const N: usize> Vector<T> for [T; N] {
    type Slice = [T];

    fn length(&self) -> usize {
        N
    }

    fn stride(&self) -> usize {
        1
    }

    unsafe fn at_u(&self, index: usize) -> &T {
        unsafe { self.get_unchecked(index) }
    }

    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(index) }
    }

    unsafe fn slice_u(&self, start: usize, end: usize) -> &[T] {
        &self[start..end]
    }

    unsafe fn slice_mut_u(&mut self, start: usize, end: usize) -> &mut [T] {
        &mut self[start..end]
    }
}

impl<T> Vector<T> for [T] {
    type Slice = [T];

    fn length(&self) -> usize {
        self.len()
    }

    fn stride(&self) -> usize {
        1
    }

    unsafe fn at_u(&self, index: usize) -> &T {
        unsafe { self.get_unchecked(index) }
    }

    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(index) }
    }

    unsafe fn slice_u(&self, start: usize, end: usize) -> &[T] {
        &self[start..end]
    }

    unsafe fn slice_mut_u(&mut self, start: usize, end: usize) -> &mut [T] {
        &mut self[start..end]
    }
}
