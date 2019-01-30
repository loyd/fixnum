use std::i64;

use failure::Fail;
use serde_derive::{Deserialize, Serialize};

use crate::Decimal;

const EXP: i32 = -9;
const COEF: i64 = 1_000_000_000;

/// Abstraction over fixed point floating numbers.
///
/// The internal representation is a fixed point decimal number,
/// i.e. a value pre-multiplied by 10^N, where N is a pre-defined number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FixedPoint(i64);

impl FixedPoint {
    pub const ZERO: FixedPoint = FixedPoint(0);
    pub const ONE: FixedPoint = FixedPoint(COEF);
    pub const MIN: FixedPoint = FixedPoint(i64::MIN);
    pub const MAX: FixedPoint = FixedPoint(i64::MAX);

    pub fn from_decimal(decimal: &Decimal) -> Result<FixedPoint, FixedPointFromDecimalError> {
        if decimal.exponent < EXP || decimal.exponent > 10 {
            return Err(FixedPointFromDecimalError::UnsupportedExponent);
        }

        let multiplier = 10i64.pow((decimal.exponent - EXP) as u32);

        decimal
            .mantissa
            .checked_mul(multiplier)
            .map(FixedPoint)
            .map_or_else(|| Err(FixedPointFromDecimalError::TooBigMantissa), Ok)
    }

    #[inline]
    pub fn checked_add(self, rhs: FixedPoint) -> Option<FixedPoint> {
        self.0.checked_add(rhs.0).map(FixedPoint)
    }

    #[inline]
    pub fn checked_sub(self, rhs: FixedPoint) -> Option<FixedPoint> {
        self.0.checked_sub(rhs.0).map(FixedPoint)
    }

    #[inline]
    pub fn checked_neg(self) -> Option<FixedPoint> {
        self.0.checked_neg().map(FixedPoint)
    }

    #[inline]
    pub fn checked_abs(self) -> Option<FixedPoint> {
        self.0.checked_abs().map(FixedPoint)
    }

    #[inline]
    pub fn checked_mul(self, rhs: i64) -> Option<FixedPoint> {
        self.0.checked_mul(rhs).map(FixedPoint)
    }
}

#[derive(Debug, Fail, PartialEq)]
pub enum FixedPointFromDecimalError {
    #[fail(display = "unsupported exponent")]
    UnsupportedExponent,
    #[fail(display = "too big mantissa")]
    TooBigMantissa,
}

impl Into<Decimal> for FixedPoint {
    fn into(self) -> Decimal {
        Decimal {
            mantissa: self.0,
            exponent: EXP,
        }
    }
}

/// Returns `FixedPoint` corresponding to the integer `value`.
impl From<i64> for FixedPoint {
    fn from(value: i64) -> Self {
        FixedPoint(value.checked_mul(COEF).expect("overflow"))
    }
}

impl<'a> From<&'a str> for FixedPoint {
    fn from(str: &'a str) -> Self {
        FixedPoint(fixed_point_from_str(str).unwrap())
    }
}

fn fixed_point_from_str(str: &str) -> Result<i64, &'static str> {
    let str = str.trim();

    let index = match str.find('.') {
        Some(index) => index,
        None => {
            let integral: i64 = str.parse().map_err(|_| "can't parse integral part")?;
            return integral.checked_mul(COEF).ok_or("overflow");
        }
    };

    let integral: i64 = str[0..index]
        .parse()
        .map_err(|_| "can't parse integral part")?;
    let fractional_str = &str[index + 1..];

    if !fractional_str.chars().all(|c| c.is_digit(10)) {
        return Err("fractional part can only contain digits");
    }

    if fractional_str.len() > EXP.abs() as usize {
        return Err("precision is too high");
    }

    let exp = 10i64.pow(fractional_str.len() as u32);

    if exp > COEF {
        return Err("precision is too high");
    }

    let fractional: i64 = fractional_str
        .parse()
        .map_err(|_| "can't parse fractional part")?;

    let final_integral = integral.checked_mul(COEF).ok_or("overflow")?;
    let signum = if integral == 0 { 1 } else { integral.signum() };
    let final_fractional = signum * COEF / exp * fractional;

    final_integral
        .checked_add(final_fractional)
        .ok_or("overflow")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::i64;

    #[test]
    fn from_decimal() {
        let p1: FixedPoint = 5.into();
        let decimal: Decimal = p1.into();

        assert_eq!(
            decimal,
            Decimal {
                mantissa: 5_000_000_000,
                exponent: -9
            }
        );

        let p2 = FixedPoint::from_decimal(&decimal);
        assert_eq!(Ok(p1), p2);
    }

    #[test]
    fn from_less_accurate_decimal() {
        let d0 = Decimal {
            mantissa: 1,
            exponent: 0,
        };

        let d1 = Decimal {
            mantissa: 1,
            exponent: 1,
        };

        assert_eq!(FixedPoint::from_decimal(&d0), Ok(FixedPoint::from(1)));
        assert_eq!(FixedPoint::from_decimal(&d1), Ok(FixedPoint::from(10)));
    }

    #[test]
    fn from_good_str() {
        assert_eq!(fixed_point_from_str("1"), Ok(1_000_000_000));
        assert_eq!(fixed_point_from_str("1.1"), Ok(1_100_000_000));
        assert_eq!(fixed_point_from_str("1.02"), Ok(1_020_000_000));
        assert_eq!(fixed_point_from_str("-1.02"), Ok(-1_020_000_000));
        assert_eq!(fixed_point_from_str("+1.02"), Ok(1_020_000_000));
        assert_eq!(
            fixed_point_from_str("123456789.123456789"),
            Ok(123_456_789_123_456_789)
        );
        assert_eq!(
            fixed_point_from_str("9223372036.854775807"),
            Ok(9_223_372_036_854_775_807)
        );
        assert_eq!(fixed_point_from_str("0.1234"), Ok(123_400_000));
    }

    #[test]
    fn from_bad_str() {
        let bad = &[
            "",
            "7.02e5",
            "a.12",
            "12.a",
            "13.0000000001",
            "13.1000000001",
            "13.9999999999999999999999999999999999999999999999999999999999999",
            "100000000000000000000000",
            "9223372036.854775808",
            "170141183460469231731687303715.884105728",
        ];

        for str in bad {
            assert!(
                fixed_point_from_str(str).is_err(),
                "must not parse '{}'",
                str
            );
        }
    }

    #[test]
    fn exp_and_coef_should_agree() {
        assert!(EXP < 0);
        assert_eq!(COEF, 10i64.pow(-EXP as u32));
    }

    #[test]
    fn mul_overflow() {
        let result = FixedPoint::MAX.checked_mul(i64::MAX);
        assert_eq!(result, None);

        let result = FixedPoint::MAX.checked_mul(i64::MIN);
        assert_eq!(result, None);
    }
}
