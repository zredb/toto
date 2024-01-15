pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for u8 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u16 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u64 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u128 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i8 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i16 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i32 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i64 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i128 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for f64 {
    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }
}
