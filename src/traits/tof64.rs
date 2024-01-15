pub trait ToF64 {
    fn to_f64(self) -> f64;
}

impl ToF64 for u8 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for u16 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for u32 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for u64 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for u128 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for i8 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for i16 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for i32 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for i64 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for i128 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl ToF64 for f32 {
    #[inline]
    fn to_f64(self) -> f64 {
        self as f64
    }
}
