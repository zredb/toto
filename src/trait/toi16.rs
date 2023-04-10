pub trait Toi16 {
    fn to_i16(self) -> i16;
}

impl Toi16 for u8 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for u16 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for u32 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for u64 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for u128 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for i8 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for i32 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for i64 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for i128 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for f32 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}

impl Toi16 for f64 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
}
