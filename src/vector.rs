use num_traits::NumAssign;

pub trait Vector<T> {
    unsafe fn at_u(&self, index: usize) -> &T;

    fn at(&self, index: usize) -> &T {
        assert!(index < self.length());
        unsafe { self.at_u(index) }
    }

    fn length(&self) -> usize;
}

pub trait VectorMut<T>: Vector<T> {
    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T;

    fn at_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.length());
        unsafe { self.at_mut_u(index) }
    }
}

impl<T, const N: usize> Vector<T> for [T; N] {
    unsafe fn at_u(&self, index: usize) -> &T {
        unsafe { self.get_unchecked(index) }
    }

    fn length(&self) -> usize {
        N
    }
}

impl<T, const N: usize> VectorMut<T> for [T; N] {
    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(index) }
    }
}

impl<T> Vector<T> for [T] {
    unsafe fn at_u(&self, index: usize) -> &T {
        unsafe { self.get_unchecked(index) }
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> VectorMut<T> for [T] {
    unsafe fn at_mut_u(&mut self, index: usize) -> &mut T {
        unsafe { self.get_unchecked_mut(index) }
    }
}

pub unsafe fn dot_u<T, A, B>(a: &A, b: &B) -> T
where
    T: Copy + NumAssign,
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());

    let mut i = 0;
    let len = a.length();
    let mut out = T::zero();

    while i < len {
        out += unsafe { *a.at_u(i) * *b.at_u(i) };
        i += 1;
    }

    out
}
