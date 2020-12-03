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

    fn cadd(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;
    type Error;

    fn csub(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;
    type Error;

    fn cmul(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;
    type Error;

    fn cdiv(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    Ceil = 1,
    Floor = -1,
}

pub trait RoundingMul<Rhs = Self> {
    type Output;
    type Error;

    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
}

pub trait RoundingDiv<Rhs = Self> {
    type Output;
    type Error;

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
