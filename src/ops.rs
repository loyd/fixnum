use std::i32;

use failure::Fail;

pub trait Numeric: Copy {
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

impl Numeric for i32 {
    const ZERO: i32 = 0;
    const ONE: i32 = 1;
    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;
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
