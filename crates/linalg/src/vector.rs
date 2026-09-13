pub trait Vector {
    type Item;
    type Slice: Vector<Item = Self::Item> + ?Sized;

    fn length(&self) -> usize;

    fn stride(&self) -> usize;

    unsafe fn at_u(&self, index: usize) -> &Self::Item;

    fn at(&self, index: usize) -> &Self::Item {
        assert!(index < self.length());
        unsafe { self.at_u(index) }
    }

    unsafe fn at_mut_u(&mut self, index: usize) -> &mut Self::Item;

    fn at_mut(&mut self, index: usize) -> &mut Self::Item {
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

impl<T, const N: usize> Vector for [T; N] {
    type Item = T;
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

impl<T> Vector for [T] {
    type Item = T;
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
