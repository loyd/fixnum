use core::convert::TryFrom;
use core::mem;

use crate::ArithmeticError;

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

pub trait Bounded {
    const MIN: Self;
    const MAX: Self;
}

pub trait CheckedAdd<Rhs = Self> {
    type Output;
    type Error;

    /// Checked addition. Returns `Err` on overflow.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::CheckedAdd};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.1".parse()?;
    /// let b: Amount = "0.2".parse()?;
    /// let c: Amount = "0.3".parse()?;
    /// assert_eq!(a.cadd(b)?, c);
    /// # Ok(()) }
    /// ```
    fn cadd(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;

    /// Saturating addition. Computes `self + rhs`, saturating at the numeric bounds
    /// ([`MIN`][MIN], [`MAX`][MAX]) instead of overflowing.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Bounded, RoundMode::*, CheckedAdd}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "1000.00002".parse()?;
    /// let b: Amount = "9222000000".parse()?;
    /// let c: Amount = "9222001000.00002".parse()?;
    /// // 1000.00002 + 9222000000 = 9222001000.00002
    /// assert_eq!(a.saturating_add(b), c);
    ///
    /// // 9222000000 + 9222000000 = MAX
    /// assert_eq!(c.saturating_add(c), Amount::MAX);
    ///
    /// let d: Amount = "-9222000000".parse()?;
    /// // -9222000000 + (-9222000000) = MIN
    /// assert_eq!(d.saturating_add(d), Amount::MIN);
    /// # Ok(()) }
    /// ```
    ///
    /// [MAX]: ./trait.Bounded.html#associatedconstant.MAX
    /// [MIN]: ./trait.Bounded.html#associatedconstant.MIN
    fn saturating_add(self, rhs: Rhs) -> Self::Output
    where
        Self: Sized,
        Rhs: PartialOrd + Zero,
        Self::Output: Bounded,
    {
        let is_rhs_negative = rhs < Rhs::ZERO;
        self.cadd(rhs).unwrap_or_else(|_| {
            if is_rhs_negative {
                Self::Output::MIN
            } else {
                Self::Output::MAX
            }
        })
    }
}

pub trait CheckedSub<Rhs = Self> {
    type Output;
    type Error;

    /// Checked subtraction. Returns `Err` on overflow.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::CheckedSub};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.3".parse()?;
    /// let b: Amount = "0.1".parse()?;
    /// let c: Amount = "0.2".parse()?;
    /// assert_eq!(a.csub(b)?, c);
    /// # Ok(()) }
    /// ```
    fn csub(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;

    /// Saturating subtraction. Computes `self - rhs`, saturating at the numeric bounds
    /// ([`MIN`][MIN], [`MAX`][MAX]) instead of overflowing.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Bounded, RoundMode::*, CheckedSub}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "9222001000.00002".parse()?;
    /// let b: Amount = "9222000000".parse()?;
    /// let c: Amount = "1000.00002".parse()?;
    /// // 9222001000.00002 - 9222000000 = 1000.00002
    /// assert_eq!(a.saturating_sub(b), c);
    ///
    /// let d: Amount = "-9222000000".parse()?;
    /// // 9222000000 - (-9222000000) = MAX
    /// assert_eq!(b.saturating_sub(d), Amount::MAX);
    ///
    /// // -9222000000 - 9222000000 = MIN
    /// assert_eq!(d.saturating_sub(b), Amount::MIN);
    /// # Ok(()) }
    /// ```
    ///
    /// [MAX]: ./trait.Bounded.html#associatedconstant.MAX
    /// [MIN]: ./trait.Bounded.html#associatedconstant.MIN
    fn saturating_sub(self, rhs: Rhs) -> Self::Output
    where
        Self: Sized,
        Rhs: PartialOrd + Zero,
        Self::Output: Bounded,
    {
        let is_rhs_negative = rhs < Rhs::ZERO;
        self.csub(rhs).unwrap_or_else(|_| {
            if is_rhs_negative {
                Self::Output::MAX
            } else {
                Self::Output::MIN
            }
        })
    }
}

pub trait CheckedMul<Rhs = Self> {
    type Output;
    type Error;

    /// Checked multiplication. Returns `Err` on overflow.
    /// This is multiplication without rounding, hence it's available only when at least one operand is integer.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::CheckedMul};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.000000001".parse()?;
    /// let b: Amount = "0.000000012".parse()?;
    /// assert_eq!(a.cmul(12)?, b);
    /// assert_eq!(12.cmul(a)?, b);
    /// # Ok(()) }
    /// ```
    fn cmul(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;

    /// Saturating multiplication. Computes `self * rhs`, saturating at the numeric bounds
    /// ([`MIN`][MIN], [`MAX`][MAX]) instead of overflowing.
    /// This is multiplication without rounding, hence it's available only when at least one operand is integer.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Zero, Bounded, RoundMode::*, CheckedMul}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.000000001".parse()?;
    /// let b: Amount = "0.000000012".parse()?;
    /// assert_eq!(a.saturating_mul(12), b);
    /// assert_eq!(12.saturating_mul(a), b);
    ///
    /// // i64::MAX * 1e-9 = MAX
    /// assert_eq!(a.saturating_mul(i64::MAX), Amount::MAX);
    ///
    /// let c: Amount = "-1.000000001".parse()?;
    /// // -1.000000001 * (SaturatingCeil) MAX = MIN
    /// assert_eq!(c.saturating_mul(i64::MAX), Amount::MIN);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [MAX]: ./trait.Bounded.html#associatedconstant.MAX
    /// [MIN]: ./trait.Bounded.html#associatedconstant.MIN
    /// [RoundMode]: ./enum.RoundMode.html
    fn saturating_mul(self, rhs: Rhs) -> Self::Output
    where
        Self: PartialOrd + Zero + Sized,
        Rhs: PartialOrd + Zero,
        Self::Output: Bounded,
    {
        let is_lhs_negative = self < Self::ZERO;
        let is_rhs_negative = rhs < Rhs::ZERO;
        self.cmul(rhs).unwrap_or_else(|_| {
            if is_lhs_negative == is_rhs_negative {
                Self::Output::MAX
            } else {
                Self::Output::MIN
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    Ceil = 1,
    Floor = -1,
}

pub trait RoundingMul<Rhs = Self> {
    type Output;
    type Error;

    /// Checked rounded multiplication. Returns `Err` on overflow.
    /// Because of provided [`RoundMode`][RoundMode] it's possible to perform across the [`FixedPoint`][FixedPoint]
    /// values.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Zero, RoundingMul, RoundMode::*}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.000000001".parse()?;
    /// let b: Amount = "0.000000002".parse()?;
    /// // 1e-9 * (Ceil) 2e-9 = 1e-9
    /// assert_eq!(a.rmul(b, Ceil)?, a);
    /// assert_eq!(b.rmul(a, Ceil)?, a);
    /// // 1e-9 * (Floor) 2e-9 = 0
    /// assert_eq!(a.rmul(b, Floor)?, Amount::ZERO);
    /// assert_eq!(b.rmul(a, Floor)?, Amount::ZERO);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [RoundMode]: ./enum.RoundMode.html
    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;

    /// Saturating rounding multiplication. Computes `self * rhs`, saturating at the numeric bounds
    /// ([`MIN`][MIN], [`MAX`][MAX]) instead of overflowing.
    /// Because of provided [`RoundMode`][RoundMode] it's possible to perform across the [`FixedPoint`][FixedPoint]
    /// values.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Zero, Bounded, RoundMode::*, RoundingMul}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.000000001".parse()?;
    /// let b: Amount = "0.000000002".parse()?;
    /// // 1e-9 * (SaturatingCeil) 2e9 = 1e-9
    /// assert_eq!(a.saturating_rmul(b, Ceil), a);
    /// // 1e-9 * (SaturatingFloor) 2e9 = 0
    /// assert_eq!(a.saturating_rmul(b, Floor), Amount::ZERO);
    ///
    /// // MIN * (SaturatingFloor) MIN = MAX
    /// assert_eq!(Amount::MIN.saturating_rmul(Amount::MIN, Floor), Amount::MAX);
    ///
    /// let c: Amount = "-1.000000001".parse()?;
    /// // -1.000000001 * (SaturatingCeil) MAX = MIN
    /// assert_eq!(c.saturating_rmul(Amount::MAX, Ceil), Amount::MIN);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [MAX]: ./trait.Bounded.html#associatedconstant.MAX
    /// [MIN]: ./trait.Bounded.html#associatedconstant.MIN
    /// [RoundMode]: ./enum.RoundMode.html
    fn saturating_rmul(self, rhs: Rhs, round_mode: RoundMode) -> Self::Output
    where
        Self: PartialOrd + Zero + Sized,
        Rhs: PartialOrd + Zero,
        Self::Output: Bounded,
    {
        let is_lhs_negative = self < Self::ZERO;
        let is_rhs_negative = rhs < Rhs::ZERO;
        self.rmul(rhs, round_mode).unwrap_or_else(|_| {
            if is_lhs_negative == is_rhs_negative {
                Self::Output::MAX
            } else {
                Self::Output::MIN
            }
        })
    }
}

pub trait RoundingDiv<Rhs = Self> {
    type Output;
    type Error;

    /// Checked rounded division. Returns `Err` on overflow or attempt to divide by zero.
    /// Because of provided [`RoundMode`][RoundMode] it's possible to perform across the [`FixedPoint`][FixedPoint]
    /// values.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Zero, RoundingDiv, RoundMode::*}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "0.000000001".parse()?;
    /// let b: Amount = "1000000000".parse()?;
    /// // 1e-9 / (Ceil) 1e9 = 1e-9
    /// assert_eq!(a.rdiv(b, Ceil)?, a);
    /// // 1e-9 / (Floor) 1e9 = 0
    /// assert_eq!(a.rdiv(b, Floor)?, Amount::ZERO);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [RoundMode]: ./enum.RoundMode.html
    fn rdiv(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
}

pub trait RoundingSqrt: Sized {
    type Error;

    /// Checked rounded square root.
    /// Returns `Err` for negative arguments.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{Zero, RoundingSqrt, RoundMode::*}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a: Amount = "81".parse()?;
    /// assert_eq!(a.rsqrt(Floor)?, "9".parse()?);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [RoundMode]: ./enum.RoundMode.html
    fn rsqrt(self, mode: RoundMode) -> Result<Self, Self::Error>;
}

pub trait Sqrt: Sized {
    type Error;

    /// Checked integer square root.
    /// Returns `Err` for negative arguments.
    ///
    /// ```
    /// use fixnum::ops::Sqrt;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(81.sqrt()?, 9);
    /// assert_eq!(80.sqrt()?, 8);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [RoundMode]: ./enum.RoundMode.html
    fn sqrt(self) -> Result<Self, Self::Error>;
}

// Impls for primitives.

macro_rules! impl_for_ints {
    ($( $int:ty ),+ $(,)?) => {
        $( impl_for_ints!(@single $int); )*
    };
    (@single $int:ty) => {
        impl Zero for $int {
            const ZERO: Self = 0;
        }

        impl One for $int {
            const ONE: Self = 1;
        }

        impl Bounded for $int {
            const MIN: Self = <$int>::MIN;
            const MAX: Self = <$int>::MAX;
        }

        impl CheckedAdd for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn cadd(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                self.checked_add(rhs).ok_or(ArithmeticError::Overflow)
            }

            #[inline]
            fn saturating_add(self, rhs: Self) -> Self::Output {
                <$int>::saturating_add(self, rhs)
            }
        }

        impl CheckedSub for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn csub(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                self.checked_sub(rhs).ok_or(ArithmeticError::Overflow)
            }

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self::Output {
                <$int>::saturating_sub(self, rhs)
            }
        }

        impl CheckedMul for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn cmul(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                self.checked_mul(rhs).ok_or(ArithmeticError::Overflow)
            }

            #[inline]
            fn saturating_mul(self, rhs: Self) -> Self::Output {
                <$int>::saturating_mul(self, rhs)
            }
        }

        impl RoundingDiv for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn rdiv(self, rhs: Self, mode: RoundMode) -> Result<Self::Output, Self::Error> {
                if rhs == 0 {
                    return Err(ArithmeticError::DivisionByZero);
                }

                let mut result = self / rhs;
                let loss = self - result * rhs;

                if loss != 0 {
                    let sign = self.signum() * rhs.signum();

                    if mode as i32 == sign as i32 {
                        result = result.checked_add(sign).ok_or(ArithmeticError::Overflow)?;
                    }
                }

                Ok(result)
            }
        }
    };
}

impl_for_ints!(i8, i16, i32, i64, i128); // TODO: unsigned?

macro_rules! impl_sqrt {
    ($( $int:ty ),+ $(,)?) => {
        $( impl_sqrt!(@single $int); )*
    };
    (@single $int:ty) => {
        impl Sqrt for $int {
            type Error = ArithmeticError;

            /// Implementation courtesy to `num` crate.
            /// @see https://github.com/rust-num/num-integer/blob/4d166cbb754244760e28ea4ce826d54fafd3e629/src/roots.rs#L278
            #[inline]
            fn sqrt(self) -> Result<Self, Self::Error> {
                #[inline]
                const fn bits<T>() -> u32 {
                    (mem::size_of::<T>() * 8) as _
                }

                #[inline]
                fn fixpoint(mut x: $int, f: impl Fn($int) -> $int) -> $int {
                    let mut xn = f(x);
                    while x < xn {
                        x = xn;
                        xn = f(x);
                    }
                    while x > xn {
                        x = xn;
                        xn = f(x);
                    }
                    x
                }

                #[cfg(feature = "std")]
                #[inline]
                fn guess(x: $int) -> $int {
                    (x as f64).sqrt() as $int
                }

                #[cfg(not(feature = "std"))]
                #[inline]
                fn guess(x: $int) -> $int {
                    #[inline]
                    fn log2_estimate(x: $int) -> u32 {
                        debug_assert!(x > 0);
                        bits::<$int>() - 1 - x.leading_zeros()
                    }

                    1 << ((log2_estimate(x) + 1) / 2)
                }

                #[allow(unused_comparisons)]
                if self < 0 {
                    return Err(ArithmeticError::DomainViolation);
                }
                if bits::<$int>() > 64 {
                    // 128-bit division is slow, so do a recursive bitwise `sqrt` until it's small enough.
                    let result = match u64::try_from(self) {
                        Ok(x) => x.sqrt()? as _,
                        Err(_) => {
                            let lo = (self >> 2u32).sqrt()? << 1;
                            let hi = lo + 1;
                            if hi * hi <= self { hi } else { lo }
                        }
                    };
                    return Ok(result);
                }
                if self < 4 {
                    return Ok((self > 0).into());
                }
                // https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method
                let next = |x: $int| (self / x + x) >> 1;
                Ok(fixpoint(guess(self), next))
            }
        }
    }
}

impl_sqrt!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
