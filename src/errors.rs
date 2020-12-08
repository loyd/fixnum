use derive_more::Display;
#[cfg(feature = "std")]
use derive_more::Error;

macro_rules! impl_error {
    ($enum:tt, $($variant:tt => $str:expr, )+) => {
        impl From<$enum> for &'static str {
            fn from(x: $enum) -> &'static str {
                match x {
                    $(
                        $enum::$variant => { $str },
                        )*
                }
            }
        }
    };
}

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Display, PartialEq)]
pub enum ArithmeticError {
    #[display(fmt = "overflow")]
    Overflow,
    #[display(fmt = "division by zero")]
    DivisionByZero,
}

impl_error!(ArithmeticError,
Overflow => "overflow",
DivisionByZero => "division by zero",
);

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Display, PartialEq)]
pub enum FromDecimalError {
    #[display(fmt = "unsupported exponent")]
    UnsupportedExponent,
    #[display(fmt = "too big mantissa")]
    TooBigMantissa,
}

impl_error!(FromDecimalError,
UnsupportedExponent => "unsupported exponent",
TooBigMantissa => "mantissa is too big",
);

#[cfg_attr(feature = "std", derive(Error))]
#[derive(Debug, Display, PartialEq)]
pub enum ConvertError {
    #[display(fmt = "overflow")]
    Overflow,
    #[display(fmt = "other: {}", _0)]
    Other(#[cfg_attr(feature = "std", error(not(source)))] &'static str),
}

impl From<ConvertError> for &'static str {
    fn from(x: ConvertError) -> &'static str {
        match x {
            ConvertError::Overflow => "overflow convert error",
            ConvertError::Other(_) => "other convert error",
        }
    }
}
