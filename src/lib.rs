use std::convert::TryFrom;
use std::str::FromStr;
use std::{fmt, i64};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ops::{
    CheckedAdd, CheckedMul, CheckedSub, Numeric, RoundMode, RoundingDiv, RoundingMul,
};

pub mod ops;
mod power_table;
#[cfg(test)]
mod tests;

const COEF: i64 = 1_000_000_000;
const COEF_128: i128 = COEF as i128;

/// Abstraction over fixed point floating numbers.
///
/// The internal representation is a fixed point decimal number,
/// i.e. a value pre-multiplied by 10^N, where N is a pre-defined number.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FixedPoint(i64);

impl FixedPoint {
    pub const EXP: i32 = -9;

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
    pub fn round_towards_zero_by(self, precision: FixedPoint) -> FixedPoint {
        self.0
            .checked_div(precision.0)
            .and_then(|v| v.checked_mul(precision.0))
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

    pub fn rounding_from_f64(value: f64) -> Result<FixedPoint, ArithmeticError> {
        let x = (value * COEF as f64).round();
        if x >= (i64::MIN as f64) && x <= (i64::MAX as f64) {
            Ok(FixedPoint(x as i64))
        } else {
            Err(ArithmeticError::Overflow)
        }
    }

    pub fn to_f64(self) -> f64 {
        (self.0 as f64) / (COEF as f64)
    }

    pub fn rounding_to_i64(self) -> i64 {
        let x = if self.0 > 0 {
            self.0 + COEF / 2
        } else {
            self.0 - COEF / 2
        };
        x / COEF
    }
}

#[derive(Debug, Error)]
#[error("value is not an integer: {0}")]
pub struct NotIntegerError(FixedPoint);

impl TryFrom<FixedPoint> for i64 {
    type Error = NotIntegerError;

    fn try_from(value: FixedPoint) -> Result<Self, Self::Error> {
        if value.0 % COEF == 0 {
            Ok(value.0 / COEF)
        } else {
            Err(NotIntegerError(value))
        }
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
        let mut frac_width = if fractional > 0 {
            -Self::EXP as usize
        } else {
            0
        };

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

impl FixedPoint {
    pub fn from_decimal(
        mantissa: i64,
        exponent: i32,
    ) -> Result<FixedPoint, FixedPointFromDecimalError> {
        if exponent < Self::EXP || exponent > 10 {
            return Err(FixedPointFromDecimalError::UnsupportedExponent);
        }

        let multiplier = 10i64.pow((exponent - Self::EXP) as u32);

        mantissa
            .checked_mul(multiplier)
            .map(FixedPoint)
            .map_or_else(|| Err(FixedPointFromDecimalError::TooBigMantissa), Ok)
    }

    pub fn from_mantissa(mantissa: i64) -> FixedPoint {
        FixedPoint(mantissa)
    }

    pub fn mantissa(self) -> i64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum FixedPointFromDecimalError {
    #[error("unsupported exponent")]
    UnsupportedExponent,
    #[error("too big mantissa")]
    TooBigMantissa,
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

/// Returns `FixedPoint` corresponding to the integer `value`.
impl From<i32> for FixedPoint {
    fn from(value: i32) -> Self {
        FixedPoint(i64::from(value).checked_mul(COEF).expect("impossible"))
    }
}

/// Returns `FixedPoint` corresponding to the integer `value`.
impl From<u32> for FixedPoint {
    fn from(value: u32) -> Self {
        FixedPoint(i64::from(value).checked_mul(COEF).expect("impossible"))
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

    if fractional_str.len() > FixedPoint::EXP.abs() as usize {
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
