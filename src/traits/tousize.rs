pub trait ToUsize {
    fn to_usize(self) -> usize;
}

impl ToUsize for u8 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl ToUsize for u16 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for u64 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for u128 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl ToUsize for i8 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for i16 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for i32 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for i64 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for i128 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl ToUsize for f32 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for f64 {
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
}
