pub trait Tou32 {
    fn to_u32(self) -> u32;
}

impl Tou32 for u8 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}

impl Tou32 for u16 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for u64 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for u128 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}

impl Tou32 for i8 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for i16 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for i32 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for i64 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for i128 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}

impl Tou32 for f32 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl Tou32 for f64 {
    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }
}
