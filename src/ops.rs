use failure::Fail;

pub trait Numeric: Copy {
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "overflow")]
pub struct Overflow;

pub trait CheckedAdd<Rhs = Self> {
    type Output;

    #[must_use]
    fn cadd(self, rhs: Rhs) -> Result<Self::Output, Overflow>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;

    #[must_use]
    fn csub(self, rhs: Rhs) -> Result<Self::Output, Overflow>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;

    #[must_use]
    fn cmul(self, rhs: Rhs) -> Result<Self::Output, Overflow>;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    #[must_use]
    fn cdiv(self, rhs: Rhs) -> Result<Self::Output, Overflow>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    AwayFromZero,
    TowardsZero,
}

pub trait RoundMul<Rhs = Self> {
    type Output;

    #[must_use]
    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Overflow>;
}

pub trait RoundDiv<Rhs = Self> {
    type Output;

    #[must_use]
    fn rdiv(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Overflow>;
}

// Impl for primitives (for tests).

macro_rules! impl_for_int {
    ($ty:ident) => {
        use std::$ty;

        impl Numeric for $ty {
            const ZERO: $ty = 0;
            const ONE: $ty = 1;
            const MIN: $ty = $ty::MIN;
            const MAX: $ty = $ty::MAX;
        }

        impl CheckedAdd for $ty {
            type Output = $ty;

            fn cadd(self, rhs: $ty) -> Result<$ty, Overflow> {
                self.checked_add(rhs).ok_or(Overflow)
            }
        }

        impl CheckedSub for $ty {
            type Output = $ty;

            fn csub(self, rhs: $ty) -> Result<$ty, Overflow> {
                self.checked_sub(rhs).ok_or(Overflow)
            }
        }
    };
}

impl_for_int!(i32);
impl_for_int!(u32);
