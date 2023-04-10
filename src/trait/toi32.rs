pub trait Toi32 {
    fn to_i32(self) -> i32;
}

impl Toi32 for u8 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for u16 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for u32 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for u64 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for u128 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for i8 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for i16 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for i64 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for i128 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for f32 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl Toi32 for f64 {
    #[inline]
    fn to_i32(self) -> i32 {
        self as i32
    }
}
