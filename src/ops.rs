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

pub trait RoundingMul<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
}

pub trait RoundingDiv<Rhs = Self> {
    type Output;
    type Error;

    #[must_use]
    fn rdiv(self, rhs: Rhs, mode: RoundMode) -> Result<Self::Output, Self::Error>;
}
