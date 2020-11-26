use std::cmp;
use std::convert::{TryFrom, TryInto};
use std::ops::Neg;

use uint::construct_uint;

use crate::ArithmeticError;

const TOTAL_BITS_COUNT: usize = 256;
const UINT_WORD_BITS_COUNT: usize = 64;
const UINT_WORDS_COUNT: usize = TOTAL_BITS_COUNT / UINT_WORD_BITS_COUNT;
const SIGN_MASK: u64 = 1 << (UINT_WORD_BITS_COUNT - 1); // MSB = 1, other are equal to 0.
const U127_MAX: u128 = u128::MAX >> 1;

// Single word has 64 bits. For 256-bit number:
// UInt words count = 256 / 64 = 4
construct_uint!{
    pub struct U256(4);
}

/// Signed 256-bit number. Works on top of U256 with help of two's complement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct I256 {
    inner: U256,
}

impl I256 {
    /// Value 0
    pub const U0: Self = Self::from_u128(0);
    /// Value 1
    pub const P1: Self = Self::from_u128(1);
    /// Value 2
    pub const P2: Self = Self::from_u128(2);
    /// Value 10
    pub const P10: Self = Self::from_u128(10);
    pub const I128_MAX: Self = Self::from_u128(U127_MAX);
    /// Value `i128::MIN`. Very useful because `abs` isn't defined for `i128::MIN`
    pub const I128_MIN: Self = Self::new(U256([0, 0, 0, SIGN_MASK]));

    const fn new(x: U256) -> Self {
        I256 {
            inner: x,
        }
    }

    const fn from_u128(x: u128) -> Self {
        Self::new(U256([x as u64, (x >> UINT_WORD_BITS_COUNT) as u64, 0, 0])) // The only way to do it const
    }

    pub fn mul(self, rhs: Self) -> Result<Self, ArithmeticError> {
        let lhs_sign = self.sign();
        let rhs_sign = rhs.sign();

        let lhs = if lhs_sign == 0 { self } else { -self };
        let rhs = if rhs_sign == 0 { rhs } else { -rhs };

        let (value, has_overflow) = lhs.inner.overflowing_mul(rhs.inner);

        if has_overflow {
            return Err(ArithmeticError::Overflow);
        }

        let result = Self::new(value);

        if lhs_sign ^ rhs_sign == 0 {
            return Ok(result);
        }

        Ok(-result)
    }

    fn abs(self) -> Self {
        if !self.is_negative() { // positive or zero
            return self;
        }
        -self
    }

    const fn is_negative(self) -> bool {
        self.sign() != 0
    }

    /// 63'rd bit shows number sign:
    /// 1 -- number < 0 (0x8000_0000_0000_0000),
    /// 0 -- number >= 0 (0).
    /// Other bits are equal to 0.
    const fn sign(self) -> u64 {
        let most_significant_word: u64 = self.words()[UINT_WORDS_COUNT - 1];
        most_significant_word & SIGN_MASK
    }

    const fn words<'a>(&'a self) -> &'a [u64; UINT_WORDS_COUNT] {
        &self.inner.0
    }
}

impl Neg for I256 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(!self.inner + 1)
    }
}

impl cmp::Ord for I256 {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => cmp::Ordering::Less,
            (false, true) => cmp::Ordering::Greater,
            _ => self.inner.cmp(&other.inner)
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
        if x == i128::MIN { // `abs` wasn't defined for this only value
            return Self::I128_MIN;
        }
        let was_negative = x < 0;
        let value = Self::new(x.abs().into());
        if was_negative {
            return -value
        }
        value
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
            let x: i128 = x.abs().inner.try_into().map_err(|_| ArithmeticError::Overflow)?;
            if was_negative {
                return Ok(-x);
            }
            Ok(x)
        }
    }

    #[test]
    fn test_u0() {
        assert_eq!(I256::U0.try_into(), Ok(0i128));
    }

    #[test]
    fn test_p1() {
        assert_eq!(I256::P1.try_into(), Ok(1i128));
    }

    #[test]
    fn test_p10() {
        assert_eq!(I256::P10.try_into(), Ok(10i128));
    }

    #[test]
    fn test_i128_min() {
        assert_eq!(I256::I128_MIN.try_into(), Ok(i128::MIN));
    }

    #[test]
    fn test_i128_max() {
        assert_eq!(I256::I128_MAX.try_into(), Ok(i128::MAX));
    }

    #[test]
    fn test_mul() {
        let n5: I256 = 5.into();
        let n7: I256 = 7.into();
        assert_eq!(n5.mul(n7), Ok(35.into()));
    }

    #[test]
    fn test_i256_from_i128() {
        fn test_i128(x: i128) {
            assert_eq!(i128::try_from(I256::from(x)), Ok(x));
        }
        test_i128(0);
        test_i128(1);
        test_i128(-1);
        test_i128(i128::MAX);
        test_i128(i128::MAX - 1);
        test_i128(i128::MIN);
        test_i128(i128::MIN + 1);
    }
}
