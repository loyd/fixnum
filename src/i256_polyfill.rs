use core::cmp::{Ordering, PartialOrd};
use core::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};

use ::i256::i256 as i256_;

use crate::{
    layout::Promotion,
    ops::{One, Zero},
    ConvertError,
};

/// A polyfill for i256.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub(crate) struct i256(pub i256_);

static_assertions::assert_eq_size!(i256, [u128; 2]);

impl i256 {
    const I128_MAX: Self = Self::from_i128(i128::MAX);
    const I128_MIN: Self = Self::from_i128(i128::MIN);
    const I64_MAX: Self = Self::from_i64(i64::MAX);
    const I64_MIN: Self = Self::from_i64(i64::MIN);
    const MIN: Self = Self(i256_::MIN);

    pub(crate) const fn from_i128(x: i128) -> Self {
        Self(i256_::from_i128(x))
    }

    const fn from_i64(x: i64) -> Self {
        Self(i256_::from_i64(x))
    }

    const fn from_i8(x: i8) -> Self {
        Self(i256_::from_i8(x))
    }

    #[cfg(test)]
    const fn new(lo: u128, hi: i128) -> Self {
        Self(i256_::new(lo, hi))
    }
}

impl Promotion for i256 {
    type Layout = i128;

    #[inline]
    fn as_layout(&self) -> Self::Layout {
        self.0.as_i128()
    }

    #[cfg(feature = "std")]
    #[inline]
    fn as_positive_f64(&self) -> f64 {
        debug_assert!(*self >= Self::ZERO);
        let hi = self.0.high() as f64;
        let lo = self.0.low() as f64;
        let b2p128 = 3.402823669209385e38;
        hi * b2p128 + lo
    }

    #[inline]
    fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }

    #[inline]
    fn mul_l(&self, rhs: Self::Layout) -> Self {
        Self(self.0.mul_iwide(rhs))
    }

    #[inline]
    fn div_l(&self, rhs: Self::Layout) -> Self {
        Self(self.0.div_iwide(rhs))
    }

    #[inline]
    fn div_rem_l(&self, rhs: Self::Layout) -> (Self, Self::Layout) {
        let (div, rem) = self.0.div_rem_iwide(rhs);
        (Self(div), rem)
    }
}

impl One for i256 {
    const ONE: Self = Self::from_i64(1);
}

impl Zero for i256 {
    const ZERO: Self = Self::from_i64(0);
}

impl Mul for i256 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for i256 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Add for i256 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for i256 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for i256 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        debug_assert_ne!(self, Self::MIN);
        Self(-self.0)
    }
}

impl Ord for i256 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for i256 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<i8> for i256 {
    #[inline]
    fn from(x: i8) -> Self {
        Self::from_i8(x)
    }
}

impl From<i64> for i256 {
    #[inline]
    fn from(x: i64) -> Self {
        Self::from_i64(x)
    }
}

impl From<i128> for i256 {
    #[inline]
    fn from(x: i128) -> Self {
        Self::from_i128(x)
    }
}

impl TryFrom<i256> for i128 {
    type Error = ConvertError;

    #[inline]
    fn try_from(x: i256) -> Result<Self, Self::Error> {
        if !(i256::I128_MIN..=i256::I128_MAX).contains(&x) {
            return Err(ConvertError::new("not in range"));
        }

        Ok(x.0.as_i128())
    }
}

impl TryFrom<i256> for i64 {
    type Error = ConvertError;

    #[inline]
    fn try_from(x: i256) -> Result<Self, Self::Error> {
        if !(i256::I64_MIN..=i256::I64_MAX).contains(&x) {
            return Err(ConvertError::new("not in range"));
        }

        Ok(x.0.as_i64())
    }
}

impl Shl<u32> for i256 {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u32> for i256 {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

// Simple smoke tests to check that the underlying implementation is adequate.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_i128() {
        assert_eq!(i128::try_from(i256::I128_MIN).unwrap(), i128::MIN);
        assert_eq!(i128::try_from(i256::I128_MAX).unwrap(), i128::MAX);
    }

    #[test]
    fn cmp() {
        use core::cmp::Ordering::{self, *};
        fn t(a: i128, b: i128, ord: Ordering) {
            let a = i256::from(a);
            let b = i256::from(b);
            assert_eq!(a.cmp(&b), ord);
            assert_eq!(b.cmp(&a), ord.reverse());
        }
        t(5, 3, Greater);
        t(-5, -5, Equal);
        t(0, -5, Greater);
    }

    #[test]
    fn from_i128() {
        fn t(x: i128) {
            assert_eq!(i128::try_from(i256::from(x)).unwrap(), x);
        }
        t(0);
        t(1);
        t(-1);
        t(i128::MAX);
        t(i128::MAX - 1);
        t(i128::MIN);
        t(i128::MIN + 1);
    }

    #[test]
    fn neg_i128() {
        fn t(x: i128) {
            assert_eq!(i128::try_from(-i256::from(x)).unwrap(), -x);
            assert_eq!(i128::try_from(-i256::from(-x)).unwrap(), x);
        }
        t(0);
        t(1);
        t(1234);
        t(123_456_789_987);
    }

    #[test]
    fn neg_i256() {
        fn t(value: i256, expected: i256) {
            let actual: i256 = -value;
            assert_eq!(actual, expected);
            assert_eq!(-actual, value);
        }
        t(i256::new(u128::MAX, i128::MAX), i256::new(1, i128::MIN));
        t(
            i256::new(u128::MAX / 2, i128::MAX / 2),
            i256::new(u128::MAX / 2 + 2, i128::MIN / 2),
        );
    }

    #[test]
    #[should_panic]
    fn neg_i256_min() {
        let _x = -i256::MIN;
    }

    #[test]
    fn add() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = i256::from(a);
            let b = i256::from(b);
            assert_eq!(i128::try_from(a + b).unwrap(), expected);
            assert_eq!(i128::try_from(b + a).unwrap(), expected);
            assert_eq!(i128::try_from((-a) + (-b)).unwrap(), -expected);
            assert_eq!(i128::try_from((-b) + (-a)).unwrap(), -expected);
        }
        t(0, 0, 0);
        t(1111, 3210, 4321);
        t(-1111, 5432, 4321);
        t(-4321, 5432, 1111);
    }

    #[test]
    fn sub() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = i256::from(a);
            let b = i256::from(b);
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
    fn mul() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = i256::from(a);
            let b = i256::from(b);
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
    fn div() {
        fn t(a: i128, b: i128, expected: i128) {
            let a = i256::from(a);
            let b = i256::from(b);
            assert_eq!(i128::try_from(a / b).unwrap(), expected);
            assert_eq!(i128::try_from((-a) / (-b)).unwrap(), expected);
        }
        t(0, 1, 0);
        t(35, 5, 7);
        t(-35, 5, -7);
    }

    #[cfg(feature = "std")]
    #[test]
    fn as_positive_f64() {
        fn t(x: i256, expected: f64) {
            assert_eq!(x.as_positive_f64(), expected);
        }
        t(0i64.into(), 0.0);
        t(1i64.into(), 1.0);
        t(i64::MAX.into(), 9.223372036854776e18);
        t(i128::MAX.into(), 1.7014118346046923e38);
        t(
            i256::from(i128::MAX) * i256::from(i128::MAX),
            2.894802230932905e76,
        );
    }
}
