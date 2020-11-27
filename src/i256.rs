use std::cmp;
use std::convert::{TryFrom, TryInto};
use std::ops::{Div, Mul, Neg, Sub};

use uint::construct_uint;

use crate::ArithmeticError;

const TOTAL_BITS_COUNT: usize = 256;
const UINT_WORD_BITS_COUNT: usize = 64;
const UINT_WORDS_COUNT: usize = TOTAL_BITS_COUNT / UINT_WORD_BITS_COUNT;
const SIGN_MASK: u64 = 1 << (UINT_WORD_BITS_COUNT - 1); // MSB = 1, other are equal to 0.

// Single word has 64 bits. For 256-bit number:
// UInt words count = 256 / 64 = 4
construct_uint! {
    pub struct U256(4);
}

/// Signed 256-bit number. Works on top of U256 with help of two's complement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct I256 {
    inner: U256,
}

impl I256 {
    /// Value `i128::MAX`
    pub const I128_MAX: Self = Self::from_u127(i128::MAX as u128);
    /// Value `i128::MIN`. Very useful because `abs` isn't defined for `i128::MIN`
    pub const I128_MIN: Self = Self::new(U256([0, 0, 0, SIGN_MASK]));

    const fn new(x: U256) -> Self {
        I256 { inner: x }
    }

    pub const fn from_u64(x: u64) -> Self {
        let mut words = [0u64; UINT_WORDS_COUNT];
        words[0] = x;
        Self::new(U256(words)) // The only way to do it const
    }

    const fn from_u127(x: u128) -> Self {
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

        if lhs_sign == rhs_sign {
            return Ok(result);
        }

        Ok(-result)
    }

    fn abs(self) -> Self {
        if !self.is_negative() {
            // positive or zero
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

impl Mul for I256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_sign = self.sign();
        let rhs_sign = rhs.sign();

        let lhs = if lhs_sign == 0 { self } else { -self };
        let rhs = if rhs_sign == 0 { rhs } else { -rhs };

        // Mustn't overflow because we're usually promoting just i128 to I256.
        let result = Self::new(lhs.inner * rhs.inner);
        if lhs_sign ^ rhs_sign == 0 {
            result
        } else {
            -result
        }
    }
}

impl Div for I256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs_sign = self.sign();
        let rhs_sign = rhs.sign();

        let lhs = if lhs_sign == 0 { self } else { -self };
        let rhs = if rhs_sign == 0 { rhs } else { -rhs };

        // Mustn't overflow because we're usually promoting just i128 to I256.
        let result = Self::new(lhs.inner / rhs.inner);
        if lhs_sign ^ rhs_sign == 0 {
            result
        } else {
            -result
        }
    }
}

impl Sub for I256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs_sign = self.sign();
        let rhs_sign = rhs.sign();

        let lhs = if lhs_sign == 0 { self } else { -self };
        let rhs = if rhs_sign == 0 { rhs } else { -rhs };

        let result = Self::new(lhs.inner - rhs.inner);
        if lhs_sign ^ rhs_sign == 0 {
            result
        } else {
            -result
        }
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
        if x == i128::MIN {
            // `abs` wasn't defined for this only value
            return Self::I128_MIN;
        }
        let was_negative = x < 0;
        let value = Self::new(x.abs().into());
        if was_negative {
            return -value;
        }
        value
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
        if was_negative {
            return Ok(-x);
        }
        Ok(x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
