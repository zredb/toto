pub mod func;
pub mod traits;

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

pub use self::traits::tof32::*;
pub use self::traits::tof64::*;
pub use self::traits::toi128::*;
pub use self::traits::toi16::*;
pub use self::traits::toi32::*;
pub use self::traits::toi64::*;
pub use self::traits::toi8::*;
pub use self::traits::tou128::*;
pub use self::traits::tou16::*;
pub use self::traits::tou32::*;
pub use self::traits::tou64::*;
pub use self::traits::tou8::*;

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
