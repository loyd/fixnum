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
    /// fn amount(s: &str) -> Amount { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(amount("0.1").cadd(amount("0.2"))?, amount("0.3"));
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
    /// fn amount(s: &str) -> Amount { s.parse().unwrap() }
    ///
    /// # fn main() {
    /// let a = amount("1000.00002");
    /// let b = amount("9222000000");
    /// let c = amount("9222001000.00002");
    /// // 1000.00002 + 9222000000 = 9222001000.00002
    /// assert_eq!(a.saturating_add(b), c);
    ///
    /// // 9222000000 + 9222000000 = MAX
    /// assert_eq!(c.saturating_add(c), Amount::MAX);
    ///
    /// let d = amount("-9222000000");
    /// // -9222000000 + (-9222000000) = MIN
    /// assert_eq!(d.saturating_add(d), Amount::MIN);
    /// # }
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
    /// fn amount(s: &str) -> FixedPoint<i64, U9> { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(amount("0.3").csub(amount("0.1"))?, amount("0.2"));
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
    /// fn amount(s: &str) -> Amount { s.parse().unwrap() }
    ///
    /// # fn main() {
    /// let a = amount("9222001000.00002");
    /// let b = amount("9222000000");
    /// let c = amount("1000.00002");
    /// // 9222001000.00002 - 9222000000 = 1000.00002
    /// assert_eq!(a.saturating_sub(b), c);
    ///
    /// let d = amount("-9222000000");
    /// // 9222000000 - (-9222000000) = MAX
    /// assert_eq!(b.saturating_sub(d), Amount::MAX);
    ///
    /// // -9222000000 - 9222000000 = MIN
    /// assert_eq!(d.saturating_sub(b), Amount::MIN);
    /// # }
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
    /// fn amount(s: &str) -> FixedPoint<i64, U9> { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(amount("0.000000001").cmul(12)?, amount("0.000000012"));
    /// assert_eq!(12.cmul(amount("0.000000001"))?, amount("0.000000012"));
    /// # Ok(()) }
    /// ```
    fn cmul(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
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
    /// fn amount(s: &str) -> FixedPoint<i64, U9> { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a = amount("0.000000001");
    /// let b = amount("0.000000002");
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
    /// fn amount(s: &str) -> Amount { s.parse().unwrap() }
    ///
    /// # fn main() {
    /// let a = amount("0.000000001");
    /// let b = amount("0.000000002");
    /// // 1e-9 * (SaturatingCeil) 2e9 = 1e-9
    /// assert_eq!(a.saturating_rmul(b, Ceil), a);
    /// // 1e-9 * (SaturatingFloor) 2e9 = 0
    /// assert_eq!(a.saturating_rmul(b, Floor), Amount::ZERO);
    ///
    /// // MIN * (SaturatingFloor) MIN = MAX
    /// assert_eq!(Amount::MIN.saturating_rmul(Amount::MIN, Floor), Amount::MAX);
    ///
    /// let c = amount("-1.000000001");
    /// // -1.000000001 * (SaturatingCeil) MAX = MIN
    /// assert_eq!(c.saturating_rmul(Amount::MAX, Ceil), Amount::MIN);
    /// # }
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
    /// fn amount(s: &str) -> FixedPoint<i64, U9> { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a = amount("0.000000001");
    /// let b = amount("1000000000");
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

// Impls for primitives.

macro_rules! impl_for_ints {
    ($int:ty, $($rest:ty),*) => {
        impl_for_ints!($int);
        impl_for_ints!($($rest),*);
    };
    ($int:ty) => {
        impl CheckedAdd for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn cadd(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                self.checked_add(rhs).ok_or(ArithmeticError::Overflow)
            }
        }

        impl CheckedSub for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn csub(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                self.checked_sub(rhs).ok_or(ArithmeticError::Overflow)
            }
        }

        impl CheckedMul for $int {
            type Output = $int;
            type Error = ArithmeticError;

            #[inline]
            fn cmul(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                self.checked_mul(rhs).ok_or(ArithmeticError::Overflow)
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

#[test]
fn it_rounds_rdiv() {
    assert_eq!(5.rdiv(2, RoundMode::Floor), Ok(2));
    assert_eq!(5.rdiv(2, RoundMode::Ceil), Ok(3));
    assert_eq!((-5).rdiv(2, RoundMode::Floor), Ok(-3));
    assert_eq!((-5).rdiv(2, RoundMode::Ceil), Ok(-2));
    assert_eq!(5.rdiv(-2, RoundMode::Floor), Ok(-3));
    assert_eq!(5.rdiv(-2, RoundMode::Ceil), Ok(-2));
    assert_eq!((-5).rdiv(-2, RoundMode::Floor), Ok(2));
    assert_eq!((-5).rdiv(-2, RoundMode::Ceil), Ok(3));
}
