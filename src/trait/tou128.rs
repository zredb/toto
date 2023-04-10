pub trait Tou128 {
    fn to_u128(self) -> u128;
}

impl Tou128 for u8 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for u16 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for u32 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for u64 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for i8 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for i16 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for i32 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for i64 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for i128 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for f32 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl Tou128 for f64 {
    #[inline]
    fn to_u128(self) -> u128 {
        self as u128
    }
}
