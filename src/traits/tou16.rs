pub trait Tou16 {
    fn to_u16(self) -> u16;
}

impl Tou16 for u8 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}

impl Tou16 for u32 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for u64 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for u128 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}

impl Tou16 for i8 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for i16 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for i32 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for i64 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for i128 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}

impl Tou16 for f32 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl Tou16 for f64 {
    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }
}
