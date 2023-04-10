pub mod func;
pub mod r#trait;

pub use self::func::fromf32::*;
pub use self::func::fromf64::*;
pub use self::func::fromi128::*;
pub use self::func::fromi16::*;
pub use self::func::fromi32::*;
pub use self::func::fromi64::*;
pub use self::func::fromi8::*;
pub use self::func::fromu128::*;
pub use self::func::fromu16::*;
pub use self::func::fromu32::*;
pub use self::func::fromu64::*;
pub use self::func::fromu8::*;

pub use self::r#trait::tof32::*;
pub use self::r#trait::tof64::*;
pub use self::r#trait::toi128::*;
pub use self::r#trait::toi16::*;
pub use self::r#trait::toi32::*;
pub use self::r#trait::toi64::*;
pub use self::r#trait::toi8::*;
pub use self::r#trait::tou128::*;
pub use self::r#trait::tou16::*;
pub use self::r#trait::tou32::*;
pub use self::r#trait::tou64::*;
pub use self::r#trait::tou8::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        //assert_eq!(0u8, 0i8);
        assert_eq!(0u8, i8tou8(0i8))
    }

    #[test]
    fn test_to() {
        assert_eq!(0u8, 0i8.to_u8())
    }
}
