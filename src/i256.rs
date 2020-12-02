use core::cmp;
use core::convert::{TryFrom, TryInto};
use core::ops::{Div, Mul, Neg, Sub};

use crate::ArithmeticError;

const TOTAL_BITS_COUNT: usize = 256;
const UINT_WORD_BITS_COUNT: usize = 64;
const UINT_WORDS_COUNT: usize = TOTAL_BITS_COUNT / UINT_WORD_BITS_COUNT;
const SIGN_MASK: u64 = 1 << (UINT_WORD_BITS_COUNT - 1); // MSB = 1, other are equal to 0.

#[allow(clippy::all)]
mod u256 {
    use uint::construct_uint;

    // Single word has 64 bits. For 256-bit number:
    // UInt words count = 256 / 64 = 4
    construct_uint! {
        pub struct U256(4);
    }
}

pub use u256::U256;

/// Signed 256-bit number. Works on top of U256 with help of two's complement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct I256 {
    inner: U256,
}

impl I256 {
    /// Value `i128::MAX`
    pub const I128_MAX: Self = Self::from_i128(i128::MAX);
    /// Value `i128::MIN`. Very useful because `abs` isn't defined for `i128::MIN`
    pub const I128_MIN: Self = Self::from_i128(i128::MIN);
    pub const MAX: Self = Self::new(U256([u64::MAX, u64::MAX, u64::MAX, !SIGN_MASK]));
    pub const MIN: Self = Self::new(U256([0, 0, 0, SIGN_MASK]));

    const fn new(x: U256) -> Self {
        I256 { inner: x }
    }

    pub const fn from_i128(x: i128) -> Self {
        let msb = if x < 0 { u64::MAX } else { 0 };
        Self::new(U256([x as u64, (x >> 64) as u64, msb, msb])) // The only way to do it const
    }

    fn abs(self) -> Self {
        if self.is_negative() {
            -self
        } else {
            self
        }
    }

    const fn is_negative(self) -> bool {
        let most_significant_word: u64 = self.words()[UINT_WORDS_COUNT - 1];
        most_significant_word & SIGN_MASK != 0
    }

    const fn words(&self) -> &[u64; UINT_WORDS_COUNT] {
        &self.inner.0
    }
}

impl Mul for I256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_was_negative = self.is_negative();
        let rhs_was_negative = rhs.is_negative();

        let lhs = if lhs_was_negative { -self } else { self };
        let rhs = if rhs_was_negative { -rhs } else { rhs };

        // Mustn't overflow because we're usually promoting just i128 to I256.
        let result = Self::new(lhs.inner * rhs.inner);
        if lhs_was_negative == rhs_was_negative {
            result
        } else {
            -result
        }
    }
}

impl Div for I256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs_was_negative = self.is_negative();
        let rhs_was_negative = rhs.is_negative();

        let lhs = if lhs_was_negative { -self } else { self };
        let rhs = if rhs_was_negative { -rhs } else { rhs };

        let result = Self::new(lhs.inner / rhs.inner);
        if lhs_was_negative == rhs_was_negative {
            result
        } else {
            -result
        }
    }
}

impl Sub for I256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x, _) = self.inner.overflowing_sub(rhs.inner);
        Self::new(x)
    }
}

impl Neg for I256 {
    type Output = Self;

    /// N.B. Neg has a single case of panicking: `-I256::MIN`
    /// Because on two's complement we always have one extra negative value
    fn neg(self) -> Self::Output {
        if self == Self::MIN {
            panic_on_overflow();
        }
        const U1: U256 = I256::from_i128(1).inner;
        // Overflow takes place when we negate zero.
        let (x, _) = (!self.inner).overflowing_add(U1);
        Self::new(x)
    }
}

impl cmp::Ord for I256 {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => cmp::Ordering::Less,
            (false, true) => cmp::Ordering::Greater,
            _ => self.inner.cmp(&other.inner),
        }
    }
}

impl cmp::PartialOrd for I256 {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<i128> for I256 {
    fn from(x: i128) -> Self {
        Self::from_i128(x)
    }
}

impl TryFrom<I256> for i128 {
    type Error = ArithmeticError;

    fn try_from(x: I256) -> Result<Self, Self::Error> {
        if x > I256::I128_MAX || x < I256::I128_MIN {
            return Err(ArithmeticError::Overflow);
        }
        if x == I256::I128_MIN {
            return Ok(i128::MIN);
        }
        let was_negative = x.is_negative();
        let x: i128 = x
            .abs()
            .inner
            .try_into()
            .map_err(|_| ArithmeticError::Overflow)?;
        Ok(if was_negative { -x } else { x })
    }
}

fn panic_on_overflow() {
    panic!("arithmetic operation overflow");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_min() {
        assert_eq!(i128::try_from(I256::I128_MIN).unwrap(), i128::MIN);
    }

    #[test]
    fn it_calculates_max() {
        assert_eq!(i128::try_from(I256::I128_MAX).unwrap(), i128::MAX);
    }

    #[test]
    fn it_compares() {
        use std::cmp::Ordering::{self, *};
        fn t(a: i128, b: i128, ord: Ordering) {
            let a = I256::from(a);
            let b = I256::from(b);
            assert_eq!(a.cmp(&b), ord);
            assert_eq!(b.cmp(&a), ord.reverse());
        }
        t(5, 3, Greater);
        t(-5, -5, Equal);
        t(0, -5, Greater);
    }

    #[test]
    fn it_converts_i256_from_i128() {
        fn it_i128(x: i128) {
            assert_eq!(i128::try_from(I256::from(x)), Ok(x));
        }
        it_i128(0);
        it_i128(1);
        it_i128(-1);
        it_i128(i128::MAX);
        it_i128(i128::MAX - 1);
        it_i128(i128::MIN);
        it_i128(i128::MIN + 1);
    }

    #[test]
    fn it_negates_i128() {
        fn t(value: i128, expected: i128) {
            let actual: I256 = -I256::from(value);
            assert_eq!(i128::try_from(actual).unwrap(), expected);
            assert_eq!(i128::try_from(-actual).unwrap(), value);
        }
        t(0, 0);
        t(1, -1);
        t(1234, -1234);
        t(123_456_789_987, -123_456_789_987);
    }

    #[test]
    fn it_negates_i256() {
        fn t(value: I256, expected: I256) {
            let actual: I256 = -value;
            assert_eq!(actual, expected);
            assert_eq!(-actual, value);
        }
        t(I256::MAX, I256::new(U256([1, 0, 0, SIGN_MASK])));
        t(
            I256::new(U256([
                0xa869_bc02_ecba_4436,
                0x5ef3_b3e7_5daa_96ce,
                0x369a_22b0_7ff5_955b,
                0x8aa9_fa9e_77c4_2900,
            ])),
            I256::new(U256([
                0x579643fd1345bbca,
                0xa10c4c18a2556931,
                0xc965dd4f800a6aa4,
                0x75560561883bd6ff,
            ])),
        );
    }

    #[test]
    #[should_panic]
    fn it_doesnt_negate_i256_min() {
        let _x = -I256::MIN;
    }

    #[test]
    fn it_subtracts() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = I256::from(a);
            let b = I256::from(b);
            assert_eq!(i128::try_from(a - b).unwrap(), expected);
            assert_eq!(i128::try_from(b - a).unwrap(), -expected);
            assert_eq!(i128::try_from((-a) - (-b)).unwrap(), -expected);
            assert_eq!(i128::try_from((-b) - (-a)).unwrap(), expected);
        }
        t(0, 0, 0);
        t(4321, 1111, 3210);
        t(4321, -1111, 5432);
        t(1111, -4321, 5432);
    }

    #[test]
    fn it_multiplies() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = I256::from(a);
            let b = I256::from(b);
            assert_eq!(i128::try_from(a * b).unwrap(), expected);
            assert_eq!(i128::try_from(b * a).unwrap(), expected);
            assert_eq!(i128::try_from((-a) * (-b)).unwrap(), expected);
            assert_eq!(i128::try_from((-b) * (-a)).unwrap(), expected);
        }
        t(0, 0, 0);
        t(7, 5, 35);
        t(-7, 5, -35);
    }

    #[test]
    fn it_divides() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = I256::from(a);
            let b = I256::from(b);
            assert_eq!(i128::try_from(a / b).unwrap(), expected);
            assert_eq!(i128::try_from((-a) / (-b)).unwrap(), expected);
        }
        t(0, 1, 0);
        t(35, 5, 7);
        t(-35, 5, -7);
    }
}
