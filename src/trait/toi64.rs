pub trait Toi64 {
    fn to_i64(self) -> i64;
}

impl Toi64 for u8 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for u16 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for u32 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for u64 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for u128 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for i8 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for i16 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for i32 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for i128 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for f32 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Toi64 for f64 {
    #[inline]
    fn to_i64(self) -> i64 {
        self as i64
    }
}
