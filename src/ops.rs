pub trait Numeric: Copy {
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

pub trait CheckedAdd<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn cadd(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn csub(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn cmul(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn cdiv(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    AwayFromZero,
    TowardsZero,
}

pub trait RoundMul<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
}

pub trait RoundDiv<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn rdiv(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
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
            type Error = ();

            fn cadd(self, rhs: $ty) -> Result<$ty, ()> {
                self.checked_add(rhs).ok_or(())
            }
        }

        impl CheckedSub for $ty {
            type Output = $ty;
            type Error = ();

            fn csub(self, rhs: $ty) -> Result<$ty, ()> {
                self.checked_sub(rhs).ok_or(())
            }
        }
    };
}

impl_for_int!(i32);
impl_for_int!(u32);
