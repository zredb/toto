pub trait Toi8 {
    fn to_i8(self) -> i8;
}

impl Toi8 for u8 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}

impl Toi8 for u16 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for u32 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for u64 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for u128 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}

impl Toi8 for i16 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for i32 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for i64 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for i128 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}

impl Toi8 for f32 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl Toi8 for f64 {
    #[inline]
    fn to_i8(self) -> i8 {
        self as i8
    }
}
