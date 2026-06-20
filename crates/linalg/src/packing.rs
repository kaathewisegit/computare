pub trait Packed {
    type Half;

    fn from_halves(lower: Self::Half, upper: Self::Half) -> Self;

    fn lower(self) -> Self;
    fn upper(self) -> Self;
}

#[cfg(target_pointer_width = "64")]
impl Packed for usize {
    type Half = u32;

    fn from_halves(lower: u32, upper: u32) -> usize {
        ((upper as usize) << 32) + lower as usize
    }

    fn lower(self) -> Self {
        self as u32 as usize
    }

    fn upper(self) -> Self {
        self >> 32
    }
}

#[cfg(target_pointer_width = "32")]
impl Packed for usize {
    type Half = u16;

    fn from_halves(lower: u16, upper: u16) -> usize {
        ((upper as usize) << 16) + lower as usize
    }

    fn lower(self) -> Self {
        self as u16 as usize
    }

    fn upper(self) -> Self {
        self >> 16
    }
}
