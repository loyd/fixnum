use core::fmt::{Display, Formatter, Result};

#[cfg(feature = "std")]
use derive_more::Error;

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Clone, Debug, PartialEq)]
pub enum ArithmeticError {
    Overflow,
    DivisionByZero,
}

impl ArithmeticError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Overflow => "overflow",
            Self::DivisionByZero => "division by zero",
        }
    }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_str())
    }
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Clone, Debug, PartialEq)]
pub enum FromDecimalError {
    UnsupportedExponent,
    TooBigMantissa,
}

impl FromDecimalError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedExponent => "unsupported exponent",
            Self::TooBigMantissa => "mantissa is too big",
        }
    }
}

impl Display for FromDecimalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_str())
    }
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Clone, Debug, PartialEq)]
pub enum ConvertError {
    Overflow,
    Other(#[cfg_attr(feature = "std", error(not(source)))] &'static str),
}

impl ConvertError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Overflow => "unsupported exponent",
            Self::Other(_) => "mantissa is too big",
        }
    }
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_str())
    }
}
