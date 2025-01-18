//! # `fixnum`
//!
//! [Fixed-point][FixedPoint] numbers with explicit rounding.
//!
//! Uses various signed integer types to store the number.
//!
//! ## Features
//! Turn them on in `Cargo.toml`:
//!
//! - `i128` — `i128` layout support which will be promoted to a polyfill for `i256` for
//!   multiplication and division.
//! - `i64` — `i64` layout support which will be promoted to `i128` for multiplication and division.
//! - `i32` — `i32` layout support which will be promoted to `i64` for multiplication and division.
//! - `i16` — `i16` layout support which will be promoted to `i32` for multiplication and division.
//! - `parity` — [`parity-scale-codec`][parity_scale_codec] support (`Encode` and `Decode`
//!   implementations).
//! - `serde` — support for `serde`.
//! - `schemars` — support for `schemars`.
//! - `std` — Enabled by default.
//!
//! At least one of `i128`, `i64`, `i32`, `i16` must be enabled.
//!
//! ## Example
//! ```
//! # #[cfg(feature = "i64")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use fixnum::{FixedPoint, typenum::U9, ops::*, fixnum};
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
//! let a: Amount = fixnum!(0.1, 9);
//! let b: Amount = fixnum!(0.2, 9);
//! assert_eq!(a.cadd(b)?, fixnum!(0.3, 9));
//!
//! let expences: Amount = fixnum!(0.000000001, 9);
//! // 1e-9 * (Floor) 1e-9 = 0
//! assert_eq!(expences.rmul(expences, RoundMode::Floor)?, fixnum!(0, 9));
//! // 1e-9 * (Ceil) 1e-9 = 1e-9
//! assert_eq!(expences.rmul(expences, RoundMode::Ceil)?, expences);
//! # Ok(()) }
//! # #[cfg(not(feature = "i64"))]
//! # fn main() {}
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
//! | [`rdiv`][rdiv] | `let result: Result<FixedPoint, ArithmeticError> = a.rdiv(b, RoundMode::Floor)` | Checked [rounding][RoundMode] division. Returns `Err` on overflow. |
//! | [`rsqrt`][rsqrt] | `let result: Result<FixedPoint, ArithmeticError> = a.rsqrt(RoundMode::Floor)` | Checked [rounding][RoundMode] square root. Returns `Err` for negative argument. |
//! | [`cneg`][cneg] | `let result: Result<FixedPoint, ArithmeticError> = a.cneg()` | Checked negation. Returns `Err` on overflow (you can't negate [`MIN` value][MIN]). |
//! | [`integral`][integral] | `let y: {integer} = x.integral(RoundMode::Floor)` | Takes [rounded][RoundMode] integral part of the number. |
//! | [`saturating_add`][saturating_add] | `let z: FixedPoint = x.saturating_add(y)` | Saturating addition |
//! | [`saturating_sub`][saturating_sub] | `let z: FixedPoint = x.saturating_sub(y)` | Saturating subtraction |
//! | [`saturating_mul`][saturating_mul] | `let z: FixedPoint = x.saturating_mul(y)` | Saturating multiplication. This is multiplication without rounding, hence it's available only when at least one operand is integer. |
//! | [`saturating_rmul`][saturating_rmul] | `let z: FixedPoint = x.saturating_rmul(y, RoundMode::Floor)` | Saturating [rounding][RoundMode] multiplication |
//!
//! ## Implementing wrapper types.
//!
//! It's possible to restrict the domain in order to reduce chance of mistakes.
//! Note that convenient [`fixnum!` macro][fixnum] works with wrapper types too.
//! ```
//! # #[cfg(feature = "i64")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
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
//! use fixnum::ops::*;
//! let size = Size(4);
//! let price = fixnum!(4.25, 9); // compile-time
//! let amount = size.cmul(price)?;
//! assert_eq!(amount, fixnum!(17, 9));
//! # Ok(()) }
//! # #[cfg(not(feature = "i64"))]
//! # fn main() {}
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
//! [rsqrt]: ./struct.FixedPoint.html#method.rsqrt
//! [RoundMode]: ./ops/enum.RoundMode.html
//! [saturating_add]: ./ops/trait.CheckedAdd.html#tymethod.saturating_add
//! [saturating_mul]: ./ops/trait.CheckedMul.html#tymethod.saturating_mul
//! [saturating_rmul]: ./ops/trait.RoundingMul.html#tymethod.saturating_rmul
//! [saturating_sub]: ./ops/trait.CheckedSub.html#tymethod.saturating_sub

#![warn(rust_2018_idioms, unreachable_pub, missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use core::{cmp::Ord, fmt, marker::PhantomData};

use typenum::Unsigned;

#[cfg(feature = "i128")]
use crate::i256_polyfill::i256;
use crate::ops::{sqrt::Sqrt, *};
use crate::string::Stringify;

mod const_fn;
mod errors;
mod float;
#[cfg(feature = "i128")]
mod i256_polyfill;
mod layout;
mod macros;
#[cfg(feature = "parity")]
mod parity;
mod power_table;
mod string;

#[cfg(not(any(feature = "i16", feature = "i32", feature = "i64", feature = "i128")))]
compile_error!("Some of the next features must be enabled: \"i128\", \"i64\", \"i32\", \"i16\"");

pub use errors::*;
pub use typenum;

pub mod ops;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

#[cfg(feature = "schemars")]
mod schemars;

#[doc(hidden)]
pub mod _priv {
    pub use crate::const_fn::*;
    pub use crate::layout::*;
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
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "i128", feature = "i64", feature = "i32", feature = "i16")))
)]
#[repr(transparent)]
pub struct FixedPoint<I, P> {
    inner: I,
    _marker: PhantomData<P>,
}

/// The number of digits in the fractional part.
pub trait Precision: Unsigned {}
impl<U: Unsigned> Precision for U {}

impl<I, P> FixedPoint<I, P> {
    /// Creates from the raw representation. `1` here is equal to `1**-P`
    pub const fn from_bits(raw: I) -> Self {
        FixedPoint {
            inner: raw,
            _marker: PhantomData,
        }
    }

    /// Returns the raw representation.
    pub const fn as_bits(&self) -> &I {
        &self.inner
    }

    /// Converts to the raw representation.
    #[inline]
    pub fn into_bits(self) -> I {
        self.inner
    }
}

macro_rules! impl_fixed_point {
    (
        $(#[$attr:meta])?
        inner = $layout:tt;
        promoted_to = $promotion:tt;
        try_from = [$($try_from:ty),*];
    ) => {const _: () = {
        use $crate::_priv::Promotion as _;

        $(#[$attr])?
        impl<P: Precision> FixedPoint<$layout, P> {
            /// The number of digits in the fractional part.
            pub const PRECISION: i32 = P::I32;
            /// The difference between `0.0` and the next larger representable number.
            pub const EPSILON: Self = Self::from_bits(1);

            const COEF: $layout = const_fn::pow10(Self::PRECISION) as _;
            const NEG_COEF: $layout = -Self::COEF;
        }

        $(#[$attr])?
        impl<P: Precision> Zero for FixedPoint<$layout, P> {
            const ZERO: Self = Self::from_bits(0);
        }

        $(#[$attr])?
        impl<P: Precision> One for FixedPoint<$layout, P> {
            const ONE: Self = Self::from_bits(Self::COEF);
        }

        $(#[$attr])?
        impl<P: Precision> Bounded for FixedPoint<$layout, P> {
            const MIN: Self = Self::from_bits($layout::MIN);
            const MAX: Self = Self::from_bits($layout::MAX);
        }

        $(#[$attr])?
        impl<P: Precision> RoundingMul for FixedPoint<$layout, P> {
            type Output = Self;
            type Error = ArithmeticError;

            #[inline]
            fn rmul(self, rhs: Self, mode: RoundMode) -> Result<Self> {
                let value = $promotion::from(self.inner).mul_l(rhs.inner);
                // `|loss| < COEF`, thus it fits in the layout.
                let (result, loss) = value.div_rem_l(Self::COEF);

                let mut result =
                    $layout::try_from(result).map_err(|_| ArithmeticError::Overflow)?;

                let sign = self.inner.signum() * rhs.inner.signum();

                let add_signed_one = if mode == RoundMode::Nearest {
                    sign as i32 >= 0 && loss + loss >= Self::COEF
                                     || loss + loss <= Self::NEG_COEF
                } else {
                    loss != 0 && mode as i32 == sign as i32
                };

                if add_signed_one {
                    result = result.checked_add(sign).ok_or(ArithmeticError::Overflow)?;
                }

                Ok(Self::from_bits(result))
            }
        }

        $(#[$attr])?
        impl<P: Precision> RoundingDiv for FixedPoint<$layout, P> {
            type Output = Self;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: Self, mode: RoundMode) -> Result<Self> {
                if rhs.inner == 0 {
                    return Err(ArithmeticError::DivisionByZero);
                }

                let numerator = $promotion::from(self.inner).mul_l(Self::COEF);
                // `|loss| < rhs`, thus it fits in the layout.
                let (result, loss) = numerator.div_rem_l(rhs.inner);

                let mut result =
                    $layout::try_from(result).map_err(|_| ArithmeticError::Overflow)?;

                if loss != 0 {
                    let sign = self.inner.signum() * rhs.inner.signum();

                    let add_signed_one = if mode == RoundMode::Nearest {
                        let loss_abs = loss.abs();
                        loss_abs + loss_abs >= rhs.inner.abs()
                    } else {
                        mode as i32 == sign as i32
                    };

                    if add_signed_one {
                        result = result.checked_add(sign).ok_or(ArithmeticError::Overflow)?;
                    }
                }

                Ok(Self::from_bits(result))
            }
        }

        $(#[$attr])?
        impl<P: Precision> RoundingDiv<$layout> for FixedPoint<$layout, P> {
            type Output = Self;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: $layout, mode: RoundMode) -> Result<Self> {
                self.inner.rdiv(rhs, mode).map(Self::from_bits)
            }
        }

        $(#[$attr])?
        impl<P: Precision> RoundingDiv<FixedPoint<$layout, P>> for $layout {
            type Output = FixedPoint<$layout, P>;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: FixedPoint<$layout, P>, mode: RoundMode) -> Result<FixedPoint<$layout, P>> {
                let lhs = FixedPoint::<$layout, P>::try_from(self).map_err(|_| ArithmeticError::Overflow)?;
                lhs.rdiv(rhs, mode)
            }
        }

        $(#[$attr])?
        impl<P: Precision> CheckedAdd for FixedPoint<$layout, P> {
            type Output = Self;
            type Error = ArithmeticError;

            #[inline]
            fn cadd(self, rhs: Self) -> Result<Self> {
                self.inner.cadd(rhs.inner).map(Self::from_bits)
            }

            #[inline]
            fn saturating_add(self, rhs: Self) -> Self::Output {
                Self::Output::from_bits(self.inner.saturating_add(rhs.inner))
            }
        }

        $(#[$attr])?
        impl<P: Precision> CheckedSub for FixedPoint<$layout, P> {
            type Output = Self;
            type Error = ArithmeticError;

            #[inline]
            fn csub(self, rhs: Self) -> Result<Self> {
                self.inner.csub(rhs.inner).map(Self::from_bits)
            }

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self::Output {
                Self::Output::from_bits(self.inner.saturating_sub(rhs.inner))
            }
        }

        $(#[$attr])?
        impl<P: Precision> CheckedMul<$layout> for FixedPoint<$layout, P> {
            type Output = Self;
            type Error = ArithmeticError;

            #[inline]
            fn cmul(self, rhs: $layout) -> Result<Self> {
                self.inner.cmul(rhs).map(Self::from_bits)
            }

            #[inline]
            fn saturating_mul(self, rhs: $layout) -> Self::Output {
                Self::Output::from_bits(self.inner.saturating_mul(rhs))
            }
        }

        $(#[$attr])?
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

        $(#[$attr])?
        impl<P: Precision> FixedPoint<$layout, P> {
            /// Returns a number representing sign of self.
            /// * `0` if the number is zero
            /// * `1` if the number is positive
            /// * `-1` if the number is negative
            #[inline]
            pub fn signum(self) -> $layout {
                self.inner.signum()
            }

            /// Returns `1/n`.
            #[inline]
            pub fn recip(self, mode: RoundMode) -> Result<Self> {
                Self::ONE.rdiv(self, mode)
            }

            /// Checked negation. Returns `Err` on overflow (you can't negate [`MIN` value][MIN]).
            ///
            /// [MIN]: ./ops/trait.Bounded.html#associatedconstant.MIN
            #[inline]
            pub fn cneg(self) -> Result<Self> {
                self.inner
                    .checked_neg()
                    .map(Self::from_bits)
                    .ok_or_else(|| ArithmeticError::Overflow)
            }

            /// Calculates `(a + b) / 2`.
            #[inline]
            pub fn half_sum(a: Self, b: Self, mode: RoundMode) -> Self {
                if a.inner.signum() != b.inner.signum() {
                    Self::from_bits(a.inner + b.inner).rdiv(2, mode).unwrap()
                } else {
                    let min = a.inner.min(b.inner);
                    let max = a.inner.max(b.inner);
                    let half_diff = (max - min).rdiv(2, mode).unwrap();
                    Self::from_bits(min + half_diff)
                }
            }

            /// Takes [rounded][RoundMode] integral part of the number.
            ///
            /// ```
            /// # #[cfg(feature = "i64")]
            /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
            /// use fixnum::{FixedPoint, typenum::U9, ops::RoundMode::*};
            ///
            /// type Amount = FixedPoint<i64, U9>;
            ///
            /// let a: Amount = "8273.519".parse()?;
            /// assert_eq!(a.integral(Floor), 8273);
            /// assert_eq!(a.integral(Nearest), 8274);
            /// assert_eq!(a.integral(Ceil), 8274);
            ///
            /// let a: Amount = "-8273.519".parse()?;
            /// assert_eq!(a.integral(Floor), -8274);
            /// assert_eq!(a.integral(Nearest), -8274);
            /// assert_eq!(a.integral(Ceil), -8273);
            /// # Ok(()) }
            /// # #[cfg(not(feature = "i64"))]
            /// # fn main() {}
            /// ```
            #[inline]
            pub fn integral(self, mode: RoundMode) -> $layout {
                let sign = self.inner.signum();
                let (mut int, frac) = (self.inner / Self::COEF, self.inner.abs() % Self::COEF);

                let add_signed_one = if mode == RoundMode::Nearest {
                    frac + frac >= Self::COEF
                } else {
                    mode as i32 == sign as i32 && frac > 0
                };

                if add_signed_one {
                    int += sign;
                }

                int
            }

            /// Returns the largest integer less than or equal to a number.
            #[inline]
            pub fn floor(self) -> Self {
                Self::from_decimal(self.integral(RoundMode::Floor), 0).unwrap()
            }

            /// Returns the smallest integer greater than or equal to a number.
            #[inline]
            pub fn ceil(self) -> Self {
                Self::from_decimal(self.integral(RoundMode::Ceil), 0).unwrap()
            }

            /// Returns the nearest integer to a number. Round half-way cases away from `0.0`.
            #[inline]
            pub fn round(self) -> Self {
                Self::from_decimal(self.integral(RoundMode::Nearest), 0).unwrap()
            }

            /// Rounds towards zero by the provided precision.
            #[inline]
            pub fn round_towards_zero_by(self, precision: Self) -> Self {
                self.inner
                    .checked_div(precision.inner)
                    .and_then(|v| v.checked_mul(precision.inner))
                    .map_or(self, Self::from_bits)
            }

            /// Returns the next power of ten:
            /// * For positive: the smallest greater than or equal to a number.
            /// * For negative: the largest less than or equal to a number.
            #[inline]
            pub fn next_power_of_ten(self) -> Result<Self> {
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

                Ok(Self::from_bits(value))
            }

            /// Returns the absolute value of a number.
            #[inline]
            pub fn abs(self) -> Result<Self> {
                if self.inner < 0 {
                    self.cneg()
                } else {
                    Ok(self)
                }
            }

            /// Checked [rounding][RoundMode] square root.
            /// Returns `Err` for negative argument.
            ///
            /// Square root of a non-negative F is a non-negative S such that:
            /// * `Floor`: `S ≤ sqrt(F)`
            /// * `Ceil`: `S ≥ sqrt(F)`
            /// * `Nearest`: `Floor` or `Ceil`, which one is closer to `sqrt(F)`
            ///
            /// ```
            /// # #[cfg(feature = "i64")]
            /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
            /// use fixnum::{ArithmeticError, FixedPoint, typenum::U9};
            /// use fixnum::ops::{Zero, RoundMode::*};
            ///
            /// type Amount = FixedPoint<i64, U9>;
            ///
            /// let a: Amount = "81".parse()?;
            /// let b: Amount = "2".parse()?;
            /// let c: Amount = "-100".parse()?;
            /// assert_eq!(a.rsqrt(Floor)?, "9".parse()?);
            /// assert_eq!(b.rsqrt(Floor)?, "1.414213562".parse()?);
            /// assert_eq!(b.rsqrt(Ceil)?, "1.414213563".parse()?);
            /// assert_eq!(c.rsqrt(Floor), Err(ArithmeticError::DomainViolation));
            /// # Ok(()) }
            /// # #[cfg(not(feature = "i64"))]
            /// # fn main() {}
            /// ```
            #[inline]
            pub fn rsqrt(self, mode: RoundMode) -> Result<Self, ArithmeticError> {
                if self.inner.is_negative() {
                    return Err(ArithmeticError::DomainViolation);
                }

                // At first we have `S_inner = S * COEF`.
                // We'd like to gain `sqrt(S) * COEF`:
                // `sqrt(S) * COEF = sqrt(S * COEF^2) = sqrt(S_inner * COEF)`
                let squared = $promotion::from(self.inner).mul_l(Self::COEF);
                let lo = squared.sqrt();

                let add_one = match mode {
                    RoundMode::Floor => false,
                    RoundMode::Nearest => {
                        // We choose to round up iff
                        //
                        //  (lo+1)^2 - squared <= squared - lo^2
                        //
                        // However, we don't want to do calculations in the promoted type,
                        // because it can be slow (`i128` and `i256`). So, we use modular
                        // arithmetic (with `2^bits(layout)` modulus) to avoid it.

                        let lo2 = lo.wrapping_mul(lo);
                        // hi^2 = (lo+1)^2 = lo^2 + 2lo + 1
                        let hi2 = lo2.wrapping_add(lo).wrapping_add(lo).wrapping_add($layout::ONE);
                        let squared = squared.as_layout();
                        hi2.wrapping_sub(squared) <= squared.wrapping_sub(lo2)
                    },
                    RoundMode::Ceil => {
                        lo.wrapping_mul(lo) != squared.as_layout()
                    },
                };

                let inner = if add_one {
                    lo + $layout::ONE
                } else {
                    lo
                };

                Ok(Self::from_bits(inner))
            }
        }

        $(#[$attr])?
        impl<P: Precision> fmt::Debug for FixedPoint<$layout, P> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut buf = Default::default();
                self.stringify(&mut buf);
                f.write_str(buf.as_str())
            }
        }

        $(#[$attr])?
        impl<P: Precision> fmt::Display for FixedPoint<$layout, P> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut buf = Default::default();
                self.stringify(&mut buf);
                f.write_str(buf.as_str())
            }
        }

        $(#[$attr])?
        impl<P: Precision> FixedPoint<$layout, P> {
            /// Creates a new number from separate mantissa and exponent.
            pub fn from_decimal(mantissa: $layout, exponent: i32) -> Result<Self, ConvertError> {
                if exponent < -Self::PRECISION || exponent > 10 {
                    return Err(ConvertError::new("unsupported exponent"));
                }

                let ten: $layout = 10;
                let multiplier = ten.pow((exponent + Self::PRECISION) as u32);

                mantissa
                    .checked_mul(multiplier)
                    .map(Self::from_bits)
                    .map_or_else(|| Err(ConvertError::new("too big mantissa")), Ok)
            }

            /// Returns a pair `(mantissa, exponent)` where `exponent`
            /// is in `[-PRECISION, max_exponent]`.
            ///
            /// Examples:
            /// * `fp!(5.5).to_decimal(0)       // => (55, -1)`
            /// * `fp!(5.5).to_decimal(-1)      // => (55, -1)`
            /// * `fp!(5.5).to_decimal(-2)      // => (550, -2)`
            /// * `fp!(50).to_decimal(0)        // => (50, 0)`
            /// * `fp!(50).to_decimal(i32::MAX) // => (5, 1)`
            ///
            /// # Panics
            /// If `max_exponent` is less than `-PRECISION`.
            pub fn to_decimal(&self, max_exponent: i32) -> ($layout, i32) {
                assert!(max_exponent >= -Self::PRECISION);

                if self.inner == 0 {
                    return (0, 0.min(max_exponent));
                }

                let mut mantissa = self.inner;
                let mut exponent = -Self::PRECISION;

                // TODO: use binary search to optimize it.
                while exponent < max_exponent && mantissa % 10 == 0 {
                    exponent += 1;
                    mantissa /= 10;
                }

                (mantissa, exponent)
            }
        }

        impl<P: Precision> From<FixedPoint<$layout, P>> for f64 {
            fn from(value: FixedPoint<$layout, P>) -> Self {
                let coef = FixedPoint::<$layout, P>::COEF;
                let integral = (value.inner / coef) as f64;
                let fractional = ((value.inner % coef) as f64) / (coef as f64);
                integral + fractional
            }
        }

        $(
            // TODO: how to make the repetition replacement trick with `$(#[$attr])`?
            impl<P: Precision> TryFrom<$try_from> for FixedPoint<$layout, P> {
                type Error = ConvertError;

                fn try_from(value: $try_from) -> Result<Self, Self::Error> {
                    $layout::try_from(value)
                        .map_err(|_| ConvertError::new("too big number"))?
                        .checked_mul(Self::COEF)
                        .map(Self::from_bits)
                        .ok_or(ConvertError::new("too big number"))
                }
            }
        )*
    };};
}

#[cfg(feature = "i16")]
impl_fixed_point!(
    #[cfg_attr(docsrs, doc(cfg(feature = "i16")))]
    inner = i16;
    promoted_to = i32;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
#[cfg(feature = "i32")]
impl_fixed_point!(
    #[cfg_attr(docsrs, doc(cfg(feature = "i32")))]
    inner = i32;
    promoted_to = i64;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
#[cfg(feature = "i64")]
impl_fixed_point!(
    #[cfg_attr(docsrs, doc(cfg(feature = "i64")))]
    inner = i64;
    promoted_to = i128;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
#[cfg(feature = "i128")]
impl_fixed_point!(
    #[cfg_attr(docsrs, doc(cfg(feature = "i128")))]
    inner = i128;
    promoted_to = i256;
    try_from = [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
);
