use core::fmt::{Display, Formatter, Result};

// TODO: once MSRV becomes 1.81, use `core::error::Error` instead.
// Also, enable doctests in CI checks even for no-std.
#[cfg(feature = "std")]
use std::error::Error;

/// Represents errors during arithmetic operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArithmeticError {
    /// A result cannot be represented.
    Overflow,
    /// Try to divide by zero.
    DivisionByZero,
    /// When someone tries to use operand out of the set of departure of the function.
    /// E.g.: when you try to compute the square root of a negative number.
    DomainViolation,
}

impl ArithmeticError {
    /// Stringify an error.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Overflow => "overflow",
            Self::DivisionByZero => "division by zero",
            Self::DomainViolation => "domain violation",
        }
    }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "std")]
impl Error for ArithmeticError {}

/// Represents errors during conversions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvertError {
    reason: &'static str,
}

impl ConvertError {
    pub(crate) fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    /// Stringify an error.
    pub const fn as_str(&self) -> &'static str {
        self.reason
    }
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "std")]
impl Error for ConvertError {}
