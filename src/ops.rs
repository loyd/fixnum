use crate::ArithmeticError;

pub trait Numeric: Copy {
    const ZERO: Self;
    const ONE: Self;
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
    /// use fixnum::{FixedPoint, typenum::U9, ops::{RoundingMul, RoundMode}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// fn amount(s: &str) -> FixedPoint<i64, U9> { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a = amount("0.000000001");
    /// let b = amount("0.000000002");
    /// // 1e-9 * (Ceil) 2e-9 = 1e-9
    /// assert_eq!(a.rmul(b, RoundMode::Ceil)?, a);
    /// assert_eq!(b.rmul(a, RoundMode::Ceil)?, a);
    /// let zero = amount("0.0");
    /// // 1e-9 * (Floor) 2e-9 = 0
    /// assert_eq!(a.rmul(b, RoundMode::Floor)?, zero);
    /// assert_eq!(b.rmul(a, RoundMode::Floor)?, zero);
    /// # Ok(()) }
    /// ```
    ///
    /// [FixedPoint]: ../struct.FixedPoint.html
    /// [RoundMode]: ./enum.RoundMode.html
    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
}

pub trait RoundingDiv<Rhs = Self> {
    type Output;
    type Error;

    /// Checked rounded division. Returns `Err` on overflow or attempt to divide by zero.
    /// Because of provided [`RoundMode`][RoundMode] it's possible to perform across the [`FixedPoint`][FixedPoint]
    /// values.
    ///
    /// ```
    /// use fixnum::{FixedPoint, typenum::U9, ops::{RoundingDiv, RoundMode}};
    ///
    /// type Amount = FixedPoint<i64, U9>;
    ///
    /// fn amount(s: &str) -> FixedPoint<i64, U9> { s.parse().unwrap() }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let a = amount("0.000000001");
    /// let b = amount("1000000000");
    /// // 1e-9 / (Ceil) 1e9 = 1e-9
    /// assert_eq!(a.rdiv(b, RoundMode::Ceil)?, a);
    /// let zero = amount("0.0");
    /// // 1e-9 / (Floor) 1e9 = 0
    /// assert_eq!(a.rdiv(b, RoundMode::Floor)?, zero);
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
