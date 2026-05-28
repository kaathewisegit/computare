mod strided;

pub use strided::StridedVectorRef;

pub trait Vector<T> {
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
}

impl<T, const N: usize> Vector<T> for [T; N] {
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
}

impl<T> Vector<T> for [T] {
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
}
