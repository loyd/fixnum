use std::convert::TryFrom;
use std::str::FromStr;
use std::{fmt, i64};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use base::ops::{CheckedAdd, CheckedMul, CheckedSub, Numeric, RoundMode, RoundingDiv, RoundingMul};

use crate::Decimal;

mod power_table;

pub(crate) const EXP: i32 = -9;
const COEF: i64 = 1_000_000_000;
const COEF_128: i128 = COEF as i128;

/// Abstraction over fixed point floating numbers.
///
/// The internal representation is a fixed point decimal number,
/// i.e. a value pre-multiplied by 10^N, where N is a pre-defined number.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct FixedPoint(i64);

impl FixedPoint {
    pub const EPSILON: FixedPoint = FixedPoint(1);
    pub const HALF: FixedPoint = FixedPoint(COEF / 2);
    pub const MAX_MINUS_ONE: FixedPoint = FixedPoint(i64::MAX - 1);
    pub const MINUS_ONE: FixedPoint = FixedPoint(-COEF);
}

#[derive(Debug, PartialEq, Error)]
pub enum ArithmeticError {
    #[error("overflow")]
    Overflow,
    #[error("division by zero")]
    DivisionByZero,
}

impl Numeric for FixedPoint {
    const ZERO: FixedPoint = FixedPoint(0);
    const ONE: FixedPoint = FixedPoint(COEF);
    const MIN: FixedPoint = FixedPoint(i64::MIN);
    const MAX: FixedPoint = FixedPoint(i64::MAX);
}

impl RoundingMul for FixedPoint {
    type Output = FixedPoint;
    type Error = ArithmeticError;

    #[inline]
    fn rmul(self, rhs: FixedPoint, mode: RoundMode) -> Result<FixedPoint, ArithmeticError> {
        // TODO(loyd): avoid 128bit arithmetic when possible,
        //      because LLVM doesn't replace 128bit division by const with multiplication.

        let value = i128::from(self.0) * i128::from(rhs.0);
        let (mut result, loss) = (value / COEF_128, value % COEF_128);

        if loss != 0 && mode as i32 == value.signum() as i32 {
            result += value.signum();
        }

        if i128::from(result as i64) != result {
            return Err(ArithmeticError::Overflow);
        }

        Ok(FixedPoint(result as i64))
    }
}

impl RoundingDiv for FixedPoint {
    type Output = FixedPoint;
    type Error = ArithmeticError;

    #[inline]
    fn rdiv(self, rhs: FixedPoint, mode: RoundMode) -> Result<FixedPoint, ArithmeticError> {
        // TODO(loyd): avoid 128bit arithmetic when possible,
        //      because LLVM doesn't replace 128bit division by const with multiplication.

        if rhs == FixedPoint::ZERO {
            return Err(ArithmeticError::DivisionByZero);
        }

        let numerator = i128::from(self.0) * COEF_128;
        let denominator = i128::from(rhs.0);

        let (mut result, loss) = (numerator / denominator, numerator % denominator);

        if loss != 0 {
            let sign = numerator.signum() * denominator.signum();

            if mode as i128 == sign {
                result += sign;
            }
        }

        if i128::from(result as i64) != result {
            return Err(ArithmeticError::Overflow);
        }

        Ok(FixedPoint(result as i64))
    }
}

impl RoundingDiv<i64> for FixedPoint {
    type Output = FixedPoint;
    type Error = ArithmeticError;

    #[inline]
    fn rdiv(self, rhs: i64, mode: RoundMode) -> Result<FixedPoint, ArithmeticError> {
        if rhs == 0 {
            return Err(ArithmeticError::DivisionByZero);
        }

        let numerator = self.0;
        let denominator = rhs;

        let (mut result, loss) = (numerator / denominator, numerator % denominator);

        if loss != 0 {
            let sign = numerator.signum() * denominator.signum();

            if mode as i64 == sign {
                result += sign;
            }
        }

        Ok(FixedPoint(result as i64))
    }
}

impl CheckedAdd for FixedPoint {
    type Output = FixedPoint;
    type Error = ArithmeticError;

    #[inline]
    fn cadd(self, rhs: FixedPoint) -> Result<FixedPoint, ArithmeticError> {
        self.0
            .checked_add(rhs.0)
            .map(FixedPoint)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl CheckedSub for FixedPoint {
    type Output = FixedPoint;
    type Error = ArithmeticError;

    #[inline]
    fn csub(self, rhs: FixedPoint) -> Result<FixedPoint, ArithmeticError> {
        self.0
            .checked_sub(rhs.0)
            .map(FixedPoint)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl CheckedMul<i64> for FixedPoint {
    type Output = FixedPoint;
    type Error = ArithmeticError;

    #[inline]
    fn cmul(self, rhs: i64) -> Result<FixedPoint, ArithmeticError> {
        self.0
            .checked_mul(rhs)
            .map(FixedPoint)
            .ok_or(ArithmeticError::Overflow)
    }
}

impl FixedPoint {
    #[inline]
    pub fn recip(self, mode: RoundMode) -> Result<FixedPoint, ArithmeticError> {
        Self::ONE.rdiv(self, mode)
    }

    #[inline]
    pub fn cneg(self) -> Result<FixedPoint, ArithmeticError> {
        self.0
            .checked_neg()
            .map(FixedPoint)
            .ok_or_else(|| ArithmeticError::Overflow)
    }

    #[inline]
    pub fn half_sum(a: FixedPoint, b: FixedPoint) -> FixedPoint {
        // TODO: optimize
        let sum = i128::from(a.0) + i128::from(b.0);
        FixedPoint((sum / 2) as i64)
    }

    #[inline]
    pub fn integral(self, mode: RoundMode) -> i64 {
        let sign = self.0.signum();
        let (int, frac) = (self.0 / COEF, self.0.abs() % COEF);

        if mode as i64 == sign && frac > 0 {
            int + sign
        } else {
            int
        }
    }

    #[inline]
    pub fn round_towards_zero_by(self, prec: FixedPoint) -> FixedPoint {
        self.0
            .checked_div(prec.0)
            .and_then(|v| v.checked_mul(prec.0))
            .map_or(self, FixedPoint)
    }

    pub fn next_power_of_ten(self) -> Result<FixedPoint, ArithmeticError> {
        if self < FixedPoint::ZERO {
            return self.cneg()?.next_power_of_ten()?.cneg();
        }

        let lz = self.0.leading_zeros() as usize;
        assert!(lz > 0, "unexpected negative value");

        let value = power_table::POWER_TABLE[lz];

        let value = if self.0 > value {
            power_table::POWER_TABLE[lz - 1]
        } else {
            value
        };

        if value == 0 {
            return Err(ArithmeticError::Overflow);
        }

        Ok(FixedPoint(value))
    }
}

impl fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = self.0.signum();
        let integral = (self.0 / COEF).abs();
        let mut fractional = (self.0 % COEF).abs();
        let mut frac_width = if fractional > 0 { -EXP as usize } else { 0 };

        while fractional > 0 && fractional % 10 == 0 {
            fractional /= 10;
            frac_width -= 1;
        }

        write!(
            f,
            "{}{}.{:0width$}",
            if sign < 0 { "-" } else { "" },
            integral,
            fractional,
            width = frac_width
        )
    }
}

impl crate::FromDecimal for FixedPoint {
    fn from_decimal(decimal: &Decimal) -> Result<FixedPoint, FixedPointFromDecimalError> {
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
}

#[derive(Debug, PartialEq, Error)]
pub enum FixedPointFromDecimalError {
    #[error("unsupported exponent")]
    UnsupportedExponent,
    #[error("too big mantissa")]
    TooBigMantissa,
}

impl From<FixedPoint> for Decimal {
    fn from(fp: FixedPoint) -> Decimal {
        Decimal {
            mantissa: fp.0,
            exponent: EXP,
        }
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum ConvertError {
    #[error("overflow")]
    Overflow,
    #[error("other: {}", _0)]
    Other(String),
}

impl TryFrom<i64> for FixedPoint {
    type Error = ConvertError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        value
            .checked_mul(COEF)
            .ok_or(Self::Error::Overflow)
            .map(FixedPoint)
    }
}

impl FromStr for FixedPoint {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fixed_point_from_str(s).map(FixedPoint)
    }
}

fn fixed_point_from_str(str: &str) -> Result<i64, ConvertError> {
    let str = str.trim();

    let index = match str.find('.') {
        Some(index) => index,
        None => {
            let integral: i64 = str.parse().map_err(|_| {
                ConvertError::Other(format!("can't parse integral part of {}", str))
            })?;
            return integral.checked_mul(COEF).ok_or(ConvertError::Overflow);
        }
    };

    let integral: i64 = str[0..index]
        .parse()
        .map_err(|_| ConvertError::Other("can't parse integral part".to_string()))?;
    let fractional_str = &str[index + 1..];

    if !fractional_str.chars().all(|c| c.is_digit(10)) {
        return Err(ConvertError::Other(format!(
            "wrong {} fractional part can only contain digits",
            str
        )));
    }

    if fractional_str.len() > EXP.abs() as usize {
        return Err(ConvertError::Other(format!(
            "precision of {} is too high",
            str
        )));
    }

    let exp = 10i64.pow(fractional_str.len() as u32);

    if exp > COEF {
        return Err(ConvertError::Other(format!(
            "precision of {} is too high",
            str
        )));
    }

    let fractional: i64 = fractional_str
        .parse()
        .map_err(|_| ConvertError::Other(format!("can't parse fractional part of {}", str)))?;

    let final_integral = integral.checked_mul(COEF).ok_or(ConvertError::Overflow)?;
    let signum = if str.as_bytes()[0] == b'-' { -1 } else { 1 };
    let final_fractional = signum * COEF / exp * fractional;

    final_integral
        .checked_add(final_fractional)
        .ok_or(ConvertError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::i64;

    use anyhow::Result;

    use crate::FromDecimal;

    fn fp(s: &str) -> Result<FixedPoint> {
        FixedPoint::from_str(s).map_err(From::from)
    }

    #[test]
    fn from_decimal() -> Result<()> {
        let p1 = fp("5")?;
        let decimal = Decimal::from(p1);

        assert_eq!(
            decimal,
            Decimal {
                mantissa: 5_000_000_000,
                exponent: -9
            }
        );

        let p2 = FixedPoint::from_decimal(&decimal);
        assert_eq!(Ok(p1), p2);

        Ok(())
    }

    #[test]
    fn from_less_accurate_decimal() -> Result<()> {
        let d0 = Decimal {
            mantissa: 1,
            exponent: 0,
        };

        let d1 = Decimal {
            mantissa: 1,
            exponent: 1,
        };

        assert_eq!(FixedPoint::from_decimal(&d0), Ok(fp("1")?));
        assert_eq!(FixedPoint::from_decimal(&d1), Ok(fp("10")?));

        Ok(())
    }

    #[test]
    fn from_good_str() -> Result<()> {
        assert_eq!(fp("1")?, FixedPoint(1_000_000_000));
        assert_eq!(fp("1.1")?, FixedPoint(1_100_000_000));
        assert_eq!(fp("1.02")?, FixedPoint(1_020_000_000));
        assert_eq!(fp("-1.02")?, FixedPoint(-1_020_000_000));
        assert_eq!(fp("+1.02")?, FixedPoint(1_020_000_000));
        assert_eq!(
            fp("123456789.123456789")?,
            FixedPoint(123_456_789_123_456_789)
        );
        assert_eq!(
            fp("9223372036.854775807")?,
            FixedPoint(9_223_372_036_854_775_807)
        );
        assert_eq!(fp("0.1234")?, FixedPoint(123_400_000));
        assert_eq!(fp("-0.1234")?, FixedPoint(-123_400_000));

        Ok(())
    }

    #[test]
    fn display() -> Result<()> {
        assert_eq!(format!("{}", fp("10.042")?), String::from("10.042"));
        assert_eq!(format!("{}", fp("10.042000")?), String::from("10.042"));
        assert_eq!(format!("{}", fp("-10.042")?), String::from("-10.042"));
        assert_eq!(format!("{}", fp("-10.042000")?), String::from("-10.042"));
        assert_eq!(
            format!("{}", fp("0.000000001")?),
            String::from("0.000000001")
        );
        assert_eq!(
            format!("{}", fp("-0.000000001")?),
            String::from("-0.000000001")
        );
        assert_eq!(format!("{}", fp("-0.000")?), String::from("0.0"));

        Ok(())
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
            assert!(fp(str).is_err(), "must not parse '{}'", str);
        }
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn exp_and_coef_should_agree() {
        assert!(EXP < 0);
        assert_eq!(COEF, 10i64.pow(-EXP as u32));
    }

    #[test]
    fn cmul_overflow() {
        let result = FixedPoint::MAX.cmul(i64::MAX);
        assert_eq!(result, Err(ArithmeticError::Overflow));

        let result = FixedPoint::MAX.cmul(i64::MIN);
        assert_eq!(result, Err(ArithmeticError::Overflow));
    }

    macro_rules! assert_rmul {
        ($a:expr, $b:expr, $mode:ident, $result:expr) => {{
            let a = FixedPoint::try_from($a)?;
            let b = FixedPoint::try_from($b)?;

            // Check the commutative property.
            assert_eq!(a.rmul(b, RoundMode::$mode), b.rmul(a, RoundMode::$mode));
            // Check the result.
            assert_eq!(
                a.rmul(b, RoundMode::$mode),
                Ok(FixedPoint::try_from($result)?)
            );
        }};
    }

    // TODO(hrls): remove
    macro_rules! assert_rmuls {
        ($a:expr, $b:expr, $mode:ident, $result:expr) => {{
            let a = fp(&format!("{}", $a))?;
            let b = fp(&format!("{}", $b))?;

            // Check the commutative property.
            assert_eq!(a.rmul(b, RoundMode::$mode), b.rmul(a, RoundMode::$mode));
            // Check the result.
            assert_eq!(
                a.rmul(b, RoundMode::$mode),
                Ok(fp(&format!("{}", $result))?)
            );
        }};
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn rmul_exact() -> Result<()> {
        assert_rmul!(525, 10, Ceil, 5250);
        assert_rmul!(525, 10, Floor, 5250);
        assert_rmul!(-525, 10, Ceil, -5250);
        assert_rmul!(-525, 10, Floor, -5250);
        assert_rmul!(-525, -10, Ceil, 5250);
        assert_rmul!(-525, -10, Floor, 5250);
        assert_rmul!(525, -10, Ceil, -5250);
        assert_rmul!(525, -10, Floor, -5250);
        assert_rmuls!(525, "0.0001", Ceil, "0.0525");
        assert_rmuls!(525, "0.0001", Floor, "0.0525");
        assert_rmuls!(-525, "0.0001", Ceil, "-0.0525");
        assert_rmuls!(-525, "0.0001", Floor, "-0.0525");
        assert_rmuls!(-525, "-0.0001", Ceil, "0.0525");
        assert_rmuls!(-525, "-0.0001", Floor, "0.0525");
        assert_rmul!(FixedPoint::MAX, 1, Ceil, FixedPoint::MAX);
        assert_rmul!(FixedPoint::MAX, 1, Floor, FixedPoint::MAX);
        assert_rmuls!(
            FixedPoint(i64::MAX / 10 * 10),
            "0.1",
            Ceil,
            FixedPoint(i64::MAX / 10)
        );
        assert_rmuls!(
            FixedPoint(i64::MAX / 10 * 10),
            "0.1",
            Floor,
            FixedPoint(i64::MAX / 10)
        );
        assert_rmuls!(1, "0.000000001", Ceil, "0.000000001");
        assert_rmuls!(1, "0.000000001", Floor, "0.000000001");
        assert_rmuls!(-1, "-0.000000001", Ceil, "0.000000001");
        assert_rmuls!(-1, "-0.000000001", Floor, "0.000000001");

        Ok(())
    }

    #[test]
    fn rmul_round() -> Result<()> {
        assert_rmuls!("0.1", "0.000000001", Ceil, "0.000000001");
        assert_rmuls!("0.1", "0.000000001", Floor, 0);
        assert_rmuls!("-0.1", "0.000000001", Ceil, 0);
        assert_rmuls!("-0.1", "0.000000001", Floor, "-0.000000001");
        assert_rmuls!("-0.1", "-0.000000001", Ceil, "0.000000001");
        assert_rmuls!("-0.1", "-0.000000001", Floor, 0);
        assert_rmuls!("0.000000001", "0.000000001", Ceil, "0.000000001");
        assert_rmuls!("0.000000001", "0.000000001", Floor, 0);
        assert_rmuls!("-0.000000001", "0.000000001", Ceil, 0);
        assert_rmuls!("-0.000000001", "0.000000001", Floor, "-0.000000001");

        Ok(())
    }

    #[test]
    fn rmul_overflow() -> Result<()> {
        let a = FixedPoint::MAX;
        let b = fp("1.1")?;
        assert_eq!(a.rmul(b, RoundMode::Ceil), Err(ArithmeticError::Overflow));

        let a = fp("140000")?;
        assert_eq!(a.rmul(a, RoundMode::Ceil), Err(ArithmeticError::Overflow));

        let a = fp("-140000")?;
        let b = fp("140000")?;
        assert_eq!(a.rmul(b, RoundMode::Ceil), Err(ArithmeticError::Overflow));

        Ok(())
    }

    #[test]
    fn rdiv_exact() -> Result<()> {
        let (numer, denom) = (fp("5")?, fp("2")?);
        let result = fp("2.5")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (fp("-5")?, fp("2")?);
        let result = fp("-2.5")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (fp("-5")?, fp("-2")?);
        let result = fp("2.5")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (fp("5")?, fp("-2")?);
        let result = fp("-2.5")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (FixedPoint::MAX, FixedPoint::MAX);
        let result = fp("1")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (fp("5")?, fp("0.2")?);
        let result = fp("25")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (fp("0.00000001")?, fp("10")?);
        let result = fp("0.000000001")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        let (numer, denom) = (fp("0.00000001")?, fp("0.1")?);
        let result = fp("0.0000001")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(result));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(result));

        Ok(())
    }

    #[test]
    fn rdiv_i64() -> Result<()> {
        fn assert_rdiv(a: &str, b: i64, mode: RoundMode, expected: &str) -> Result<()> {
            let a = fp(a)?;
            let expected = fp(expected)?;
            assert_eq!(a.rdiv(b, mode).unwrap(), expected);
            Ok(())
        }

        assert_rdiv("2.4", 2, RoundMode::Ceil, "1.2")?;
        assert_rdiv("7", 3, RoundMode::Floor, "2.333333333")?;
        assert_rdiv("7", 3, RoundMode::Ceil, "2.333333334")?;
        assert_rdiv("-7", 3, RoundMode::Floor, "-2.333333334")?;
        assert_rdiv("-7", 3, RoundMode::Ceil, "-2.333333333")?;
        assert_rdiv("-7", -3, RoundMode::Floor, "2.333333333")?;
        assert_rdiv("-7", -3, RoundMode::Ceil, "2.333333334")?;
        assert_rdiv("7", -3, RoundMode::Floor, "-2.333333334")?;
        assert_rdiv("7", -3, RoundMode::Ceil, "-2.333333333")?;
        assert_rdiv("0", 5, RoundMode::Ceil, "0")?;
        assert_rdiv("0.000000003", 2, RoundMode::Ceil, "0.000000002")?;
        assert_rdiv("0.000000003", 2, RoundMode::Floor, "0.000000001")?;
        assert_rdiv("0.000000003", 7, RoundMode::Floor, "0")?;
        assert_rdiv("0.000000003", 7, RoundMode::Ceil, "0.000000001")?;
        assert_rdiv("0.000000001", 7, RoundMode::Ceil, "0.000000001")?;
        Ok(())
    }

    #[test]
    fn rdiv_round() -> Result<()> {
        let (numer, denom) = (fp("100")?, fp("3")?);
        let ceil = fp("33.333333334")?;
        let floor = fp("33.333333333")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(ceil));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(floor));

        let (numer, denom) = (fp("-100")?, fp("3")?);
        let ceil = fp("-33.333333333")?;
        let floor = fp("-33.333333334")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(ceil));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(floor));

        let (numer, denom) = (fp("-100")?, fp("-3")?);
        let ceil = fp("33.333333334")?;
        let floor = fp("33.333333333")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(ceil));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(floor));

        let (numer, denom) = (fp("100")?, fp("-3")?);
        let ceil = fp("-33.333333333")?;
        let floor = fp("-33.333333334")?;
        assert_eq!(numer.rdiv(denom, RoundMode::Ceil), Ok(ceil));
        assert_eq!(numer.rdiv(denom, RoundMode::Floor), Ok(floor));

        Ok(())
    }

    #[test]
    fn rdiv_division_by_zero() -> Result<()> {
        assert_eq!(
            FixedPoint::MAX.rdiv(FixedPoint::ZERO, RoundMode::Ceil),
            Err(ArithmeticError::DivisionByZero)
        );

        Ok(())
    }

    #[test]
    fn rdiv_overflow() -> Result<()> {
        assert_eq!(
            FixedPoint::MAX.rdiv(fp("0.5")?, RoundMode::Ceil),
            Err(ArithmeticError::Overflow)
        );
        Ok(())
    }

    #[test]
    fn half_sum() -> Result<()> {
        fn t(a: &str, b: &str, r: &str) -> Result<()> {
            let a = fp(a)?;
            let b = fp(b)?;
            let r = fp(r)?;
            assert_eq!(FixedPoint::half_sum(a, b), r);
            Ok(())
        }

        t("1", "3", "2")?;
        t("1", "2", "1.5")?;
        t("9000", "9050", "9025")?;
        t("9000", "-9000", "0")?;
        t("9000000000", "9000000002", "9000000001")?;
        t(
            "9000000000.000000001",
            "-9000000000.000000005",
            "-0.000000002",
        )?;
        t("7.123456789", "7.123456788", "7.123456788")?;

        Ok(())
    }

    #[test]
    fn integral() -> Result<()> {
        let a = fp("0.0001")?;
        assert_eq!(a.integral(RoundMode::Floor), 0);
        assert_eq!(a.integral(RoundMode::Ceil), 1);

        let b = fp("-0.0001")?;
        assert_eq!(b.integral(RoundMode::Floor), -1);
        assert_eq!(b.integral(RoundMode::Ceil), 0);

        let c = FixedPoint::ZERO;
        assert_eq!(c.integral(RoundMode::Floor), 0);
        assert_eq!(c.integral(RoundMode::Ceil), 0);

        let d = fp("2.0001")?;
        assert_eq!(d.integral(RoundMode::Floor), 2);
        assert_eq!(d.integral(RoundMode::Ceil), 3);

        let e = fp("-2.0001")?;
        assert_eq!(e.integral(RoundMode::Floor), -3);
        assert_eq!(e.integral(RoundMode::Ceil), -2);

        Ok(())
    }

    #[test]
    fn round_towards_zero_by() -> Result<()> {
        let a = fp("1234.56789")?;
        assert_eq!(a.round_towards_zero_by(fp("100")?), fp("1200")?);
        assert_eq!(a.round_towards_zero_by(fp("10")?), fp("1230")?);
        assert_eq!(a.round_towards_zero_by(fp("1")?), fp("1234")?);
        assert_eq!(a.round_towards_zero_by(fp("0.1")?), fp("1234.5")?);
        assert_eq!(a.round_towards_zero_by(fp("0.01")?), fp("1234.56")?);
        assert_eq!(a.round_towards_zero_by(fp("0.001")?), fp("1234.567")?);
        assert_eq!(a.round_towards_zero_by(fp("0.0001")?), fp("1234.5678")?);
        assert_eq!(a.round_towards_zero_by(fp("0.00001")?), fp("1234.56789")?);

        let b = fp("-1234.56789")?;
        assert_eq!(b.round_towards_zero_by(fp("100")?), fp("-1200")?);
        assert_eq!(b.round_towards_zero_by(fp("10")?), fp("-1230")?);
        assert_eq!(b.round_towards_zero_by(fp("1")?), fp("-1234")?);
        assert_eq!(b.round_towards_zero_by(fp("0.1")?), fp("-1234.5")?);
        assert_eq!(b.round_towards_zero_by(fp("0.01")?), fp("-1234.56")?);
        assert_eq!(b.round_towards_zero_by(fp("0.001")?), fp("-1234.567")?);
        assert_eq!(b.round_towards_zero_by(fp("0.0001")?), fp("-1234.5678")?);
        assert_eq!(b.round_towards_zero_by(fp("0.00001")?), fp("-1234.56789")?);

        Ok(())
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn next_power_of_ten() -> Result<()> {
        assert_eq!(fp("0")?.next_power_of_ten()?, fp("0.000000001")?);
        assert_eq!(fp("0.000000001")?.next_power_of_ten()?, fp("0.000000001")?);
        assert_eq!(fp("0.000000002")?.next_power_of_ten()?, fp("0.00000001")?);
        assert_eq!(fp("0.000000009")?.next_power_of_ten()?, fp("0.00000001")?);
        assert_eq!(fp("0.00000001")?.next_power_of_ten()?, fp("0.00000001")?);
        assert_eq!(fp("0.00000002")?.next_power_of_ten()?, fp("0.0000001")?);
        assert_eq!(fp("0.1")?.next_power_of_ten()?, fp("0.1")?);
        assert_eq!(fp("0.100000001")?.next_power_of_ten()?, fp("1")?);
        assert_eq!(fp("1")?.next_power_of_ten()?, fp("1")?);
        assert_eq!(fp("2")?.next_power_of_ten()?, fp("10")?);
        assert_eq!(fp("1234567")?.next_power_of_ten()?, fp("10000000")?);
        assert_eq!(
            fp("923372036.854775807")?.next_power_of_ten()?,
            fp("1000000000")?
        );
        assert_eq!(
            fp("9223372036.854775807")?.next_power_of_ten(),
            Err(ArithmeticError::Overflow)
        );

        assert_eq!(
            fp("-0.000000001")?.next_power_of_ten()?,
            fp("-0.000000001")?
        );
        assert_eq!(fp("-0.000000002")?.next_power_of_ten()?, fp("-0.00000001")?);
        assert_eq!(fp("-0.000000009")?.next_power_of_ten()?, fp("-0.00000001")?);
        assert_eq!(fp("-0.00000001")?.next_power_of_ten()?, fp("-0.00000001")?);
        assert_eq!(fp("-0.00000002")?.next_power_of_ten()?, fp("-0.0000001")?);
        assert_eq!(fp("-0.1")?.next_power_of_ten()?, fp("-0.1")?);
        assert_eq!(fp("-0.2")?.next_power_of_ten()?, fp("-1")?);
        assert_eq!(fp("-1")?.next_power_of_ten()?, fp("-1")?);
        assert_eq!(fp("-0.100000001")?.next_power_of_ten()?, fp("-1")?);
        assert_eq!(fp("-1234567")?.next_power_of_ten()?, fp("-10000000")?);
        assert_eq!(
            fp("-923372036.854775808")?.next_power_of_ten()?,
            fp("-1000000000")?
        );
        assert_eq!(
            fp("-9223372036.854775807")?.next_power_of_ten(),
            Err(ArithmeticError::Overflow)
        );
        assert_eq!(
            fp("-9223372036.854775808")?.next_power_of_ten(),
            Err(ArithmeticError::Overflow)
        );

        Ok(())
    }
}
