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
//! ## Features
//! Turn them on in `Cargo.toml`:
//!
//! - `i128` — `i128` layout support which will be promoted to internally implemented `I256` for
//!   multiplication and division.
//! - `parity` — [`parity-scale-codec`][parity_scale_codec] support (`Encode` and `Decode`
//!   implementations).
//!
//! ## Example
//! ```
//! use fixnum::{FixedPoint, typenum::U9, ops::{CheckedAdd, RoundingMul, RoundMode::*, Zero}};
//!
//! /// Signed fixed point amount over 64 bits, 9 decimal places.
//! ///
//! /// MAX = (2 ^ (BITS_COUNT - 1) - 1) / 10 ^ PRECISION =
//! ///     = (2 ^ (64 - 1) - 1) / 1e9 =
//! ///     = 9223372036.854775807 ~ 9.2e9
//! /// ERROR_MAX = 0.5 / (10 ^ PRECISION) =
//! ///           = 0.5 / 1e9 =
//! ///           = 5e-10
//! type Amount = FixedPoint<i64, U9>;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let a: Amount = "0.1".parse()?;
//! let b: Amount = "0.2".parse()?;
//! assert_eq!(a.cadd(b)?, "0.3".parse()?);
//!
//! let expences: Amount = "0.000000001".parse()?;
//! // 1e-9 * (Floor) 1e-9 = 0
//! assert_eq!(expences.rmul(expences, Floor)?, Amount::ZERO);
//! // 1e-9 * (Ceil) 1e-9 = 1e-9
//! assert_eq!(expences.rmul(expences, Ceil)?, expences);
//! # Ok(()) }
//! ```
//!
//! ## Available operations
//!
//! | Method | Example (pseudo-code) | Description |
//! | ------ | --------------------- | ----------- |
//! | [`cadd`][cadd] | `let result: Result<FixedPoint, ArithmeticError> = a.cadd(b)` | Checked addition. Returns `Err` on overflow. |
//! | [`csub`][csub] | `let result: Result<FixedPoint, ArithmeticError> = a.csub(b)` | Checked subtraction. Returns `Err` on overflow. |
//! | [`cmul`][cmul] | `let result: Result<FixedPoint, ArithmeticError> = a.cmul(b)` | Checked multiplication. Returns `Err` on overflow. This is multiplication without rounding, hence it's available only when at least one operand is integer. |
//! | [`rmul`][rmul] | `let result: Result<FixedPoint, ArithmeticError> = a.rmul(b, RoundMode::Ceil)` | Checked rounding multiplication. Returns `Err` on overflow. Because of provided [`RoundMode`][RoundMode] it's possible across the [`FixedPoint`][FixedPoint] values. |
//! | [`rdiv`][rdiv] | `let result: Result<FixedPoint, ArithmeticError> = a.rdiv(b, RoundMode::Floor)` | Checked rounding division. Returns `Err` on overflow. Because of provided [`RoundMode`][RoundMode] it's possible across the [`FixedPoint`][FixedPoint] values. |
//! | [`cneg`][cneg] | `let result: Result<FixedPoint, ArithmeticError> = a.cneg()` | Checked negation. Returns `Err` on overflow (you can't negate [`MIN` value][MIN]). |
//! | [`integral`][integral] | `let y: {integer} = x.integral(RoundMode::Floor)` | Takes [rounded][RoundMode] integral part of the number. |
//! | [`saturating_add`][saturating_add] | `let z: FixedPoint = x.saturating_add(y)` | Saturating addition |
//! | [`saturating_sub`][saturating_sub] | `let z: FixedPoint = x.saturating_sub(y)` | Saturating subtraction |
//! | [`saturating_mul`][saturating_mul] | `let z: FixedPoint = x.saturating_mul(y)` | Saturating multiplication. This is multiplication without rounding, hence it's available only when at least one operand is integer. |
//! | [`saturating_rmul`][saturating_rmul] | `let z: FixedPoint = x.saturating_rmul(y, RoundMode::Floor)` | Saturating rounding multiplication |
//!
//! ## Implementing wrapper types.
//! It's possible to restrict the domain in order to reduce chance of mistakes.
//! Note that convenient [`fixnum!` macro][fixnum] works with wrapper types too.
//! ```
//! use derive_more::From;
//! use fixnum::{impl_op, typenum::U9, FixedPoint, fixnum};
//!
//! type Fp64 = FixedPoint<i64, U9>;
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, From)]
//! struct Size(i32);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, From)]
//! struct Price(Fp64);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, From)]
//! struct PriceDelta(Fp64);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, From)]
//! struct Amount(Fp64);
//! #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, From)]
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
//! let size = Size(4);
//! let price = fixnum!(4.25, 9); // compile-time
//! let amount = size.cmul(price)?;
//! assert_eq!(amount, fixnum!(17, 9));
//! # Ok(()) }
//! ```
//!
//! [cadd]: ./ops/trait.CheckedAdd.html#tymethod.cadd
//! [cneg]: ./struct.FixedPoint.html#method.cneg
//! [csub]: ./ops/trait.CheckedSub.html#tymethod.csub
//! [cmul]: ./ops/trait.CheckedMul.html#tymethod.cmul
//! [fixnum]: ./macro.fixnum.html
//! [FixedPoint]: ./struct.FixedPoint.html
//! [integral]: ./struct.FixedPoint.html#method.integral
//! [MIN]: ./ops/trait.Bounded.html#associatedconstant.MIN
//! [parity_scale_codec]: https://docs.rs/parity-scale-codec
//! [rdiv]: ./ops/trait.RoundingDiv.html#tymethod.rdiv
//! [rmul]: ./ops/trait.RoundingMul.html#tymethod.rmul
//! [RoundMode]: ./ops/enum.RoundMode.html
//! [saturating_add]: ./ops/trait.CheckedAdd.html#tymethod.saturating_add
//! [saturating_mul]: ./ops/trait.CheckedMul.html#tymethod.saturating_mul
//! [saturating_rmul]: ./ops/trait.RoundingMul.html#tymethod.saturating_rmul
//! [saturating_sub]: ./ops/trait.CheckedSub.html#tymethod.saturating_sub

#![warn(rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]
use core::convert::TryFrom;
use core::str::FromStr;
use core::{fmt, i64, marker::PhantomData};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typenum::Unsigned;

#[cfg(feature = "i128")]
use crate::i256::I256;
use crate::ops::{
    Bounded, CheckedAdd, CheckedMul, CheckedSub, One, RoundMode, RoundingDiv, RoundingMul, Zero,
};
pub use typenum;

mod const_fn;
mod errors;
#[cfg(feature = "i128")]
mod i256;
mod macros;
pub mod ops;
#[cfg(feature = "parity")]
mod parity;
mod power_table;
#[cfg(test)]
mod tests;

pub use errors::*;

#[doc(hidden)]
pub mod _priv {
    pub use crate::const_fn::*;
    pub use crate::macros::Operand;
    pub use crate::ops::*;
}

type Result<T, E = ArithmeticError> = core::result::Result<T, E>;

/// Abstraction over fixed point numbers of arbitrary (but only compile-time specified) size
/// and precision.
///
/// The internal representation is a fixed point decimal number,
/// an integer value pre-multiplied by `10 ^ PRECISION`,
/// where `PRECISION` is a compile-time-defined decimal places count.
///
/// Maximal possible value: `MAX = (2 ^ (BITS_COUNT - 1) - 1) / 10 ^ PRECISION`
/// Maximal possible calculation error: `ERROR_MAX = 0.5 / (10 ^ PRECISION)`
///
/// E.g. for `i64` with 9 decimal places:
///
/// ```text
/// MAX = (2 ^ (64 - 1) - 1) / 1e9 = 9223372036.854775807 ~ 9.2e9
/// ERROR_MAX = 0.5 / 1e9 = 5e-10
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedPoint<I, P> {
    inner: I,
    _marker: PhantomData<P>,
}

pub trait Precision: Unsigned {}
impl<U: Unsigned> Precision for U {}

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

    #[inline]
    pub fn into_bits(self) -> I {
        self.inner
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

            const COEF: $layout = const_fn::pow10(Self::PRECISION) as _;
            const COEF_PROMOTED: $promotion = $convert(Self::COEF) as _;
        }

        impl<P: Precision> Zero for FixedPoint<$layout, P> {
            const ZERO: Self = Self::from_bits(0);
        }

        impl<P: Precision> One for FixedPoint<$layout, P> {
            const ONE: Self = Self::from_bits(Self::COEF);
        }

        impl<P: Precision> Bounded for FixedPoint<$layout, P> {
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

            #[inline]
            fn saturating_add(self, rhs: Self) -> Self::Output {
                Self::Output::from_bits(self.inner.saturating_add(rhs.inner))
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

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self::Output {
                Self::Output::from_bits(self.inner.saturating_sub(rhs.inner))
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

            #[inline]
            fn saturating_mul(self, rhs: $layout) -> Self::Output {
                Self::Output::from_bits(self.inner.saturating_mul(rhs))
            }
        }

        impl<P: Precision> CheckedMul<FixedPoint<$layout, P>> for $layout {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn cmul(self, rhs: FixedPoint<$layout, P>) -> Result<FixedPoint<$layout, P>> {
                rhs.cmul(self)
            }

            #[inline]
            fn saturating_mul(self, rhs: FixedPoint<$layout, P>) -> Self::Output {
                Self::Output::from_bits(self.saturating_mul(rhs.inner))
            }
        }

        impl<P: Precision> FixedPoint<$layout, P> {
            #[inline]
            pub fn recip(self, mode: RoundMode) -> Result<FixedPoint<$layout, P>> {
                Self::ONE.rdiv(self, mode)
            }

            /// Checked negation. Returns `Err` on overflow (you can't negate [`MIN` value][MIN]).
            ///
            /// [MIN]: ./ops/trait.Bounded.html#associatedconstant.MIN
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

            /// Takes [rounded][RoundMode] integral part of the number.
            ///
            /// ```
            /// use fixnum::{FixedPoint, typenum::U9, ops::RoundMode::*};
            ///
            /// type Amount = FixedPoint<i64, U9>;
            ///
            /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
            /// let a: Amount = "8273.519".parse()?;
            /// assert_eq!(a.integral(Floor), 8273);
            /// assert_eq!(a.integral(Ceil), 8274);
            ///
            /// let a: Amount = "-8273.519".parse()?;
            /// assert_eq!(a.integral(Floor), -8274);
            /// assert_eq!(a.integral(Ceil), -8273);
            /// # Ok(()) }
            /// ```
            ///
            /// [RoundMode]: ./ops/enum.RoundMode.html
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

            // TODO: make this operation checked
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
