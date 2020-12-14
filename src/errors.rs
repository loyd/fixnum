#[cfg(feature = "std")]
use derive_more::Error;

macro_rules! impl_error {
    ($enum:ty, $($variant:pat => $str:expr, )+) => {
        impl $enum {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $( $variant => { $str }, )*
                }
            }
        }

        impl ::core::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    Overflow,
    DivisionByZero,
}

impl_error!(ArithmeticError,
    ArithmeticError::Overflow => "overflow",
    ArithmeticError::DivisionByZero => "division by zero",
);

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, PartialEq)]
pub enum FromDecimalError {
    UnsupportedExponent,
    TooBigMantissa,
}

impl_error!(FromDecimalError,
    FromDecimalError::UnsupportedExponent => "unsupported exponent",
    FromDecimalError::TooBigMantissa => "mantissa is too big",
);

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, PartialEq)]
pub enum ConvertError {
    Overflow,
    Other(#[cfg_attr(feature = "std", error(not(source)))] &'static str),
}

impl_error!(ConvertError,
    ConvertError::Overflow => "unsupported exponent",
    ConvertError::Other(_) => "mantissa is too big",
);
