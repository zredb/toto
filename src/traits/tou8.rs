pub trait Tou8 {
    fn to_u8(self) -> u8;
}

impl Tou8 for u16 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Tou8 for u32 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for u64 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for u128 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Tou8 for i8 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for i16 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for i32 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for i64 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for i128 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Tou8 for f32 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl Tou8 for f64 {
    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}
