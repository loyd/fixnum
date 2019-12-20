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
