#[repr(C)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub const fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}
