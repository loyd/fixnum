//! # `fixnum`
//!
//! [Fixed-point][FixedPoint] numbers with explicit rounding.
//!
//! Uses various signed integer types to store the number. The following are available by default:
//!
//! - `i16` — promotes to `i32` for multiplication and division,
//! - `i32` — promotes to `i64` (for mul, div),
//! - `i64` — promotes to `i128` (for mul, div).
//!
//! There's also support for `i128` layout which will be promoted to internally implemented `I256` for multiplication
//! and division — this is available under the `i128` feature.
//!
//! ## Example
//! ```
//! use fixnum::{FixedPoint, typenum::U9, ops::{CheckedAdd, RoundingMul, RoundMode::*}};
//!
//! /// Signed fixed point amount over 64 bits, 9 decimal places.
//! ///
//! /// MAX = (2 ** (BITS_COUNT - 1) - 1) / 10 ** PRECISION =
//! ///     = (2 ** (64 - 1) - 1) / 1e9 =
//! ///     = 9223372036.854775807 ~ 9.2e9
//! /// ERROR_MAX = 0.5 / (10 ** PRECISION) =
//! ///           = 0.5 / 1e9 =
//! ///           = 5e-10
//! type Amount = FixedPoint<i64, U9>;
//!
//! fn amount(s: &str) -> Amount { s.parse().unwrap() }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! assert_eq!(amount("0.1").cadd(amount("0.2"))?, amount("0.3"));
//! let expences: Amount = amount("0.000000001");
//! assert_eq!(expences.rmul(expences, Floor)?, amount("0.0"));
//! // 1e-9 * (Ceil) 1e-9 = 1e-9
//! assert_eq!(expences.rmul(expences, Ceil)?, expences);
//! # Ok(()) }
//! ```
//!
//! ## Available operations
//!
//! | Method Name | Example (pseudo-code) | Description |
//! | ----------- | --------------------- | ----------- |
//! | [`cadd`][cadd] | `let result: Result<FixedPoint, ArithmeticError> = a.cadd(b)` | Checked addition. Returns `Err` on overflow. |
//! | [`csub`][csub] | `let result: Result<FixedPoint, ArithmeticError> = a.csub(b)` | Checked subtraction. Returns `Err` on overflow. |
//! | [`cmul`][cmul] | `let result: Result<FixedPoint, ArithmeticError> = a.cmul(b)` | Checked multiplication. Returns `Err` on overflow. This is multiplication without rounding, hence it's available only when at least one operand is integer. |
//! | [`rmul`][rmul] | `let result: Result<FixedPoint, ArithmeticError> = a.rmul(b, RoundMode::Ceil)` | Checked rounded multiplication. Returns `Err` on overflow. Because of provided [`RoundMode`][RoundMode] it's possible across the [`FixedPoint`][FixedPoint] values. |
//! | [`rdiv`][rdiv] | `let result: Result<FixedPoint, ArithmeticError> = a.rdiv(b, RoundMode::Floor)` | Checked rounded division. Returns `Err` on overflow. Because of provided [`RoundMode`][RoundMode] it's possible across the [`FixedPoint`][FixedPoint] values. |
//!
//! ## Implementing wrapper types.
//! It's possible to restrict the domain in order to reduce chance of mistakes:
//! ```
//! #[macro_use]
//! extern crate fixnum;
//! use fixnum::{impl_op, typenum::U9, FixedPoint};
//!
//! type Fp64 = FixedPoint<i64, U9>;
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
//! struct Size(i32);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
//! struct Price(Fp64);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
//! struct PriceDelta(Fp64);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
//! struct Amount(Fp64);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
//! struct Ratio(Fp64);
//!
//! impl_op!(Size [cadd] Size = Size);
//! impl_op!(Size [csub] Size = Size);
//! impl_op!(Size [rdiv] Size = Ratio);
//! impl_op!(Size [cmul] Price = Amount);
//! impl_op!(Price [csub] Price = PriceDelta);
//! impl_op!(Price [cadd] PriceDelta = Price);
//! impl_op!(Price [rdiv] Price = Ratio);
//! impl_op!(Price [rmul] Ratio = Price);
//! impl_op!(PriceDelta [cadd] PriceDelta = PriceDelta);
//! impl_op!(Amount [cadd] Amount = Amount);
//! impl_op!(Amount [csub] Amount = Amount);
//!
//! // Use it.
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use fixnum::ops::*;
//! let size = Size(2);
//! let price = Price("4.25".parse()?);
//! let amount = size.cmul(price)?;
//! assert_eq!(amount, Amount("8.5".parse()?));
//! # Ok(()) }
//! ```
//!
//! [cadd]: ./ops/trait.CheckedAdd.html#tymethod.cadd
//! [csub]: ./ops/trait.CheckedSub.html#tymethod.csub
//! [cmul]: ./ops/trait.CheckedMul.html#tymethod.cmul
//! [rmul]: ./ops/trait.RoundingMul.html#tymethod.rmul
//! [rdiv]: ./ops/trait.RoundingDiv.html#tymethod.rdiv
//! [FixedPoint]: ./struct.FixedPoint.html
//! [RoundMode]: ./ops/enum.RoundMode.html

#![warn(rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
use core::convert::TryFrom;
use core::str::FromStr;
use core::{fmt, i64, marker::PhantomData};

use derive_more::Display;
#[cfg(feature = "std")]
use derive_more::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typenum::Unsigned;

#[cfg(feature = "i128")]
use crate::i256::I256;
use crate::ops::{
    CheckedAdd, CheckedMul, CheckedSub, Numeric, RoundMode, RoundingDiv, RoundingMul,
};
pub use typenum;

#[cfg(feature = "i128")]
mod i256;
mod macros;
pub mod ops;
mod power_table;
#[cfg(test)]
mod tests;

#[doc(hidden)]
pub mod _priv {
    pub use crate::macros::Operand;
    pub use crate::ops::*;
}

type Result<T, E = ArithmeticError> = core::result::Result<T, E>;

/// Abstraction over fixed point floating numbers.
///
/// The internal representation is a fixed point decimal number,
/// i.e. a value pre-multiplied by 10^N, where N is a pre-defined number.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedPoint<I, P> {
    inner: I,
    _marker: PhantomData<P>,
}

pub trait Precision: Unsigned {}
impl<U: Unsigned> Precision for U {}

/////////////////////////////////////////////////////////

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, derive_more::Display, PartialEq)]
pub enum ArithmeticError {
    #[display(fmt = "overflow")]
    Overflow,
    #[display(fmt = "division by zero")]
    DivisionByZero,
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Display, PartialEq)]
pub enum FromDecimalError {
    #[display(fmt = "unsupported exponent")]
    UnsupportedExponent,
    #[display(fmt = "too big mantissa")]
    TooBigMantissa,
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Display, PartialEq)]
pub enum ConvertError {
    #[display(fmt = "overflow")]
    Overflow,
    #[display(fmt = "other: {}", _0)]
    Other(#[cfg_attr(feature = "std", error(not(source)))] &'static str),
}

macro_rules! pow10 {
    ($convert:expr, $rhs:expr) => {{
        let mut result = 1;
        let mut i = $rhs;
        while i > 0 {
            result *= 10;
            i -= 1;
        }
        $convert(result)
    }};
}

impl<I, P> FixedPoint<I, P> {
    pub const fn from_bits(raw: I) -> Self {
        FixedPoint {
            inner: raw,
            _marker: PhantomData,
        }
    }

    pub const fn as_bits(&self) -> &I {
        &self.inner
    }
}

macro_rules! impl_fixed_point {
    (
        inner = $layout:tt;
        promoted_to = $promotion:tt;
        convert = $convert:expr;
        try_from = [$($try_from:ty),*];
    ) => {
        impl<P: Precision> FixedPoint<$layout, P> {
            pub const PRECISION: i32 = P::I32;
            pub const EPSILON: Self = Self::from_bits(1);

            const COEF: $layout = pow10!(identity, Self::PRECISION);
            const COEF_PROMOTED: $promotion = pow10!($convert, Self::PRECISION);

            // TODO
            //pub const HALF: Self = Self::from_bits(Self::COEF / 2);
            //pub const MAX_MINUS_ONE: Self = Self::from_bits(i64::MAX - 1);
            //pub const MINUS_ONE: Self = Self::from_bits(-COEF);
        }

        impl<P: Precision> Numeric for FixedPoint<$layout, P> {
            const ZERO: Self = Self::from_bits(0);
            const ONE: Self = Self::from_bits(Self::COEF);
            const MIN: Self = Self::from_bits($layout::MIN);
            const MAX: Self = Self::from_bits($layout::MAX);
        }

        impl<P: Precision> RoundingMul for FixedPoint<$layout, P> {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn rmul(self, rhs: Self, mode: RoundMode) -> Result<Self> {
                // TODO(loyd): avoid 128bit arithmetic when possible,
                //      because LLVM doesn't replace 128bit division by const with multiplication.

                let value = $promotion::from(self.inner) * $promotion::from(rhs.inner);
                // TODO: replace with multiplication by constant
                let result = value / Self::COEF_PROMOTED;
                let loss = value - result * Self::COEF_PROMOTED;
                let sign = self.inner.signum() * rhs.inner.signum();

                let mut result =
                    $layout::try_from(result).map_err(|_| ArithmeticError::Overflow)?;

                if loss != $convert(0) && mode as i32 == sign as i32 {
                    result = result.checked_add(sign).ok_or(ArithmeticError::Overflow)?;
                }

                Ok(Self::from_bits(result))
            }
        }

        impl<P: Precision> RoundingDiv for FixedPoint<$layout, P> {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: Self, mode: RoundMode) -> Result<Self> {
                // TODO(loyd): avoid 128bit arithmetic when possible,
                //      because LLVM doesn't replace 128bit division by const with multiplication.

                if rhs.inner == 0 {
                    return Err(ArithmeticError::DivisionByZero);
                }

                let numerator = $promotion::from(self.inner) * Self::COEF_PROMOTED;
                let denominator = $promotion::from(rhs.inner);
                let result = numerator / denominator;
                let loss = numerator - result * denominator;

                let mut result =
                    $layout::try_from(result).map_err(|_| ArithmeticError::Overflow)?;

                if loss != $convert(0) {
                    let sign = self.inner.signum() * rhs.inner.signum();

                    if mode as i32 == sign as i32 {
                        result = result.checked_add(sign).ok_or(ArithmeticError::Overflow)?;
                    }
                }

                Ok(Self::from_bits(result))
            }
        }

        impl<P: Precision> RoundingDiv<$layout> for FixedPoint<$layout, P> {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: $layout, mode: RoundMode) -> Result<FixedPoint<$layout, P>> {
                if rhs == 0 {
                    return Err(ArithmeticError::DivisionByZero);
                }

                let numerator = self.inner;
                let denominator = rhs;
                let mut result = numerator / denominator;
                let loss = numerator - result * denominator;

                if loss != 0 {
                    let sign = numerator.signum() * denominator.signum();

                    if mode as i32 == sign as i32 {
                        result = result.checked_add(sign).ok_or(ArithmeticError::Overflow)?;
                    }
                }

                Ok(Self::from_bits(result))
            }
        }

        impl<P: Precision> RoundingDiv<FixedPoint<$layout, P>> for $layout {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: FixedPoint<$layout, P>, mode: RoundMode) -> Result<FixedPoint<$layout, P>> {
                let lhs = FixedPoint::<$layout, P>::try_from(self).map_err(|_| ArithmeticError::Overflow)?;
                lhs.rdiv(rhs, mode)
            }
        }

        impl<P: Precision> CheckedAdd for FixedPoint<$layout, P> {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn cadd(self, rhs: FixedPoint<$layout, P>) -> Result<FixedPoint<$layout, P>> {
                self.inner
                    .checked_add(rhs.inner)
                    .map(Self::from_bits)
                    .ok_or(ArithmeticError::Overflow)
            }
        }

        impl<P: Precision> CheckedSub for FixedPoint<$layout, P> {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn csub(self, rhs: FixedPoint<$layout, P>) -> Result<FixedPoint<$layout, P>> {
                self.inner
                    .checked_sub(rhs.inner)
                    .map(Self::from_bits)
                    .ok_or(ArithmeticError::Overflow)
            }
        }

        impl<P: Precision> CheckedMul<$layout> for FixedPoint<$layout, P> {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn cmul(self, rhs: $layout) -> Result<FixedPoint<$layout, P>> {
                self.inner
                    .checked_mul(rhs)
                    .map(Self::from_bits)
                    .ok_or(ArithmeticError::Overflow)
            }
        }

        impl<P: Precision> CheckedMul<FixedPoint<$layout, P>> for $layout {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn cmul(self, rhs: FixedPoint<$layout, P>) -> Result<FixedPoint<$layout, P>> {
                rhs.cmul(self)
            }
        }

        impl<P: Precision> FixedPoint<$layout, P> {
            #[inline]
            pub fn recip(self, mode: RoundMode) -> Result<FixedPoint<$layout, P>> {
                Self::ONE.rdiv(self, mode)
            }

            #[inline]
            pub fn cneg(self) -> Result<FixedPoint<$layout, P>> {
                self.inner
                    .checked_neg()
                    .map(Self::from_bits)
                    .ok_or_else(|| ArithmeticError::Overflow)
            }

            #[inline]
            pub fn half_sum(
                a: FixedPoint<$layout, P>,
                b: FixedPoint<$layout, P>,
            ) -> FixedPoint<$layout, P> {
                if a.inner.signum() != b.inner.signum() {
                    Self::from_bits((a.inner + b.inner) / 2)
                } else {
                    let min = a.inner.min(b.inner);
                    let max = a.inner.max(b.inner);
                    Self::from_bits(min + (max - min) / 2)
                }
            }

            #[inline]
            pub fn integral(self, mode: RoundMode) -> $layout {
                let sign = self.inner.signum();
                let (int, frac) = (self.inner / Self::COEF, self.inner.abs() % Self::COEF);

                if mode as i32 == sign as i32 && frac > 0 {
                    int + sign
                } else {
                    int
                }
            }

            #[inline]
            pub fn round_towards_zero_by(
                self,
                precision: FixedPoint<$layout, P>,
            ) -> FixedPoint<$layout, P> {
                self.inner
                    .checked_div(precision.inner)
                    .and_then(|v| v.checked_mul(precision.inner))
                    .map_or(self, Self::from_bits)
            }

            pub fn next_power_of_ten(self) -> Result<FixedPoint<$layout, P>> {
                if self.inner < 0 {
                    return self.cneg()?.next_power_of_ten()?.cneg();
                }

                let lz = self.inner.leading_zeros() as usize;
                assert!(lz > 0, "unexpected negative value");

                let value = power_table::$layout[lz];

                let value = if self.inner > value {
                    power_table::$layout[lz - 1]
                } else {
                    value
                };

                if value == 0 {
                    return Err(ArithmeticError::Overflow);
                }

                // TODO
                Ok(Self::from_bits(value as $layout))
            }

            #[cfg(feature = "std")]
            pub fn rounding_from_f64(value: f64) -> Result<FixedPoint<$layout, P>> {
                let x = (value * Self::COEF as f64).round();
                if x >= ($layout::MIN as f64) && x <= ($layout::MAX as f64) {
                    Ok(Self::from_bits(x as $layout))
                } else {
                    Err(ArithmeticError::Overflow)
                }
            }

            pub fn to_f64(self) -> f64 {
                (self.inner as f64) / (Self::COEF as f64)
            }

            // TODO
            pub fn rounding_to_i64(self) -> i64 {
                let x = if self.inner > 0 {
                    self.inner + Self::COEF / 2
                } else {
                    self.inner - Self::COEF / 2
                };
                (x / Self::COEF) as i64
            }
        }

        impl<P: Precision> fmt::Debug for FixedPoint<$layout, P> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl<P: Precision> fmt::Display for FixedPoint<$layout, P> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let sign = self.inner.signum();
                let integral = (self.inner / Self::COEF).abs();
                let mut fractional = (self.inner % Self::COEF).abs();
                let mut frac_width = if fractional > 0 {
                    Self::PRECISION as usize
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

        impl<P: Precision> FixedPoint<$layout, P> {
            pub fn from_decimal(
                mantissa: $layout,
                exponent: i32,
            ) -> Result<FixedPoint<$layout, P>, FromDecimalError> {
                if exponent < -Self::PRECISION || exponent > 10 {
                    return Err(FromDecimalError::UnsupportedExponent);
                }

                let ten: $layout = 10;
                let multiplier = ten.pow((exponent + Self::PRECISION) as u32);

                mantissa
                    .checked_mul(multiplier)
                    .map(Self::from_bits)
                    .map_or_else(|| Err(FromDecimalError::TooBigMantissa), Ok)
            }
        }

        $(
            impl<P: Precision> TryFrom<$try_from> for FixedPoint<$layout, P> {
                type Error = ConvertError;

                fn try_from(value: $try_from) -> Result<Self, Self::Error> {
                    $layout::try_from(value)
                        .map_err(|_| ConvertError::Overflow)?
                        .checked_mul(Self::COEF)
                        .map(Self::from_bits)
                        .ok_or(ConvertError::Overflow)
                }
            }
        )*

        impl<P: Precision> FromStr for FixedPoint<$layout, P> {
            type Err = ConvertError;

            fn from_str(str: &str) -> Result<Self, Self::Err> {
                let str = str.trim();
                let coef = Self::COEF;

                let index = match str.find('.') {
                    Some(index) => index,
                    None => {
                        let integral: $layout = str.parse().map_err(|_| {
                            ConvertError::Other("can't parse integral part of the str")
                        })?;
                        return integral
                            .checked_mul(coef)
                            .ok_or(ConvertError::Overflow)
                            .map(Self::from_bits);
                    }
                };

                let integral: $layout = str[0..index]
                    .parse()
                    .map_err(|_| ConvertError::Other("can't parse integral part"))?;
                let fractional_str = &str[index + 1..];

                if !fractional_str.chars().all(|c| c.is_digit(10)) {
                    return Err(ConvertError::Other("can't parse fractional part: must contain digits only"));
                }

                if fractional_str.len() > Self::PRECISION.abs() as usize {
                    return Err(ConvertError::Other("requested precision is too high"));
                }

                let ten: $layout = 10;
                let exp = ten.pow(fractional_str.len() as u32);

                if exp > coef {
                    return Err(ConvertError::Other("requested precision is too high"));
                }

                let fractional: $layout = fractional_str.parse().map_err(|_| {
                    ConvertError::Other("can't parse fractional part")
                })?;

                let final_integral = integral.checked_mul(coef).ok_or(ConvertError::Overflow)?;
                let signum = if str.as_bytes()[0] == b'-' { -1 } else { 1 };
                let final_fractional = signum * coef / exp * fractional;

                final_integral
                    .checked_add(final_fractional)
                    .map(Self::from_bits)
                    .ok_or(ConvertError::Overflow)
            }
        }
    };
}

const fn identity<T>(x: T) -> T {
    x
}

impl_fixed_point!(
    inner = i16;
    promoted_to = i32;
    convert = identity;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
impl_fixed_point!(
    inner = i32;
    promoted_to = i64;
    convert = identity;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
impl_fixed_point!(
    inner = i64;
    promoted_to = i128;
    convert = identity;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
#[cfg(feature = "i128")]
impl_fixed_point!(
    inner = i128;
    promoted_to = I256;
    convert = I256::from_i128;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
