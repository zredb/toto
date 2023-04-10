pub trait Tou64 {
    fn to_u64(self) -> u64;
}

impl Tou64 for u8 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for u16 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for u32 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for u128 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for i8 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for i16 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for i32 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for i64 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for i128 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for f32 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl Tou64 for f64 {
    #[inline]
    fn to_u64(self) -> u64 {
        self as u64
    }
}
