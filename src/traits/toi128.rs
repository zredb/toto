pub trait Toi128 {
    fn to_i128(self) -> i128;
}

impl Toi128 for u8 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for u16 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for u32 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for u64 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for u128 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}

impl Toi128 for i8 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for i16 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for i32 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for i64 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}

impl Toi128 for f32 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
impl Toi128 for f64 {
    #[inline]
    fn to_i128(self) -> i128 {
        self as i128
    }
}
