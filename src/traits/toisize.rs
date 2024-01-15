pub trait ToIsize {
    fn to_isize(self) -> isize;
}

impl ToIsize for u8 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}

impl ToIsize for u16 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for u64 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for u128 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}

impl ToIsize for i8 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for i16 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for i32 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for i64 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for i128 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}

impl ToIsize for f32 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
impl ToIsize for f64 {
    #[inline]
    fn to_isize(self) -> isize {
        self as isize
    }
}
