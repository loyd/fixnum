#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    AwayFromZero,
    TowardsZero,
}

pub trait RoundMul<Rhs = Self> {
    type Output;

    #[must_use]
    fn rmul(self, rhs: Rhs, mode: RoundMode) -> Self::Output;
}

pub trait RoundDiv<Rhs = Self> {
    type Output;

    #[must_use]
    fn rdiv(self, rhs: Rhs, mode: RoundMode) -> Self::Output;
}
