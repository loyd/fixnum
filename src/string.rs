use core::str::{self, FromStr};

use crate::{ConvertError, FixedPoint, Precision};

#[allow(unreachable_pub)]
pub trait Stringify {
    fn stringify(&self, buf: &mut StrBuf);
}

macro_rules! impl_for {
    ($layout:tt) => {
        impl<P: Precision> FromStr for FixedPoint<$layout, P> {
            type Err = ConvertError;

            /// Parses a string slice into a fixed point.
            /// If the value cannot be represented, it will be rounded to the nearest value.
            ///
            /// Use `from_str_exact` to parse without rounding.
            fn from_str(str: &str) -> Result<Self, Self::Err> {
                Self::parse_str_with_scientific::<false>(str)
            }
        }

        impl<P: Precision> FixedPoint<$layout, P> {
            /// Parses a string slice into a fixed point.
            /// If the value cannot be represented then this will return an error.
            ///
            /// Use the `FromStr` instance to parse with rounding.
            pub fn from_str_exact(str: &str) -> Result<Self, ConvertError> {
                Self::parse_str_with_scientific::<true>(str)
            }

            fn parse_str_without_scientific<const EXACT: bool>(
                str: &str,
            ) -> Result<Self, ConvertError> {
                let str = str.trim();

                let (integral_str, mut fractional_str) = if let Some(parts) = str.split_once('.') {
                    parts
                } else {
                    return str
                        .parse::<$layout>()
                        .map_err(|_| ConvertError::new("can't parse integer"))?
                        .try_into();
                };

                let integral: $layout = integral_str
                    .parse()
                    .map_err(|_| ConvertError::new("can't parse integral part"))?;

                if !fractional_str.chars().all(|c| c.is_digit(10)) {
                    return Err(ConvertError::new(
                        "can't parse fractional part: must contain digits only",
                    ));
                }

                let signum = if str.as_bytes()[0] == b'-' { -1 } else { 1 };
                let prec = Self::PRECISION as usize; // TODO: negative precision?

                if EXACT {
                    if fractional_str.len() > Self::PRECISION.abs() as usize {
                        return Err(ConvertError::new("requested precision is too high"));
                    }
                }

                let round = if !EXACT && fractional_str.len() > prec {
                    let extra = fractional_str.as_bytes()[prec];
                    fractional_str = &fractional_str[..prec];
                    Some(signum).filter(|_| extra >= b'5')
                } else {
                    None
                };

                let ten: $layout = 10;
                let exp = ten.pow(fractional_str.len() as u32);

                if EXACT && exp > Self::COEF {
                    return Err(ConvertError::new("requested precision is too high"));
                }

                debug_assert!(exp <= Self::COEF);

                let fractional: $layout = fractional_str
                    .parse()
                    .map_err(|_| ConvertError::new("can't parse fractional part"))?;

                let final_integral = integral
                    .checked_mul(Self::COEF)
                    .ok_or(ConvertError::new("too big integral"))?;

                let mut final_fractional = signum * Self::COEF / exp * fractional;
                if let Some(round) = round {
                    debug_assert!(!EXACT);
                    final_fractional += round;
                }

                final_integral
                    .checked_add(final_fractional)
                    .map(Self::from_bits)
                    .ok_or_else(|| ConvertError::new("too big number"))
            }

            fn parse_str_with_scientific<const EXACT: bool>(
                str: &str,
            ) -> Result<Self, ConvertError> {
                let str = str.trim();

                let (integral_and_fractional_str, exponent_str) = if let Some(exponent_char) =
                    str.chars().find(|c| *c == 'e' || *c == 'E')
                {
                    if let Some(parts) = str.split_once(exponent_char) {
                        parts
                    } else {
                        // This error should never happen because `exponent_char` already found.
                        return Err(ConvertError::new("unable to split string by exponent char"));
                    }
                } else {
                    return Self::parse_str_without_scientific::<EXACT>(str);
                };

                let mut exponent: i32 = exponent_str
                    .parse()
                    .map_err(|_| ConvertError::new("can't parse exponent"))?;

                let (mut integral_str, mut fractional_str) =
                    if let Some((integral_str, fractional_str)) =
                        integral_and_fractional_str.split_once('.')
                    {
                        (integral_str.to_owned(), fractional_str.to_owned())
                    } else {
                        (integral_and_fractional_str.to_owned(), "".to_owned())
                    };

                if !integral_str.is_empty() {
                    let mut chars = integral_str.chars();
                    let first_char = chars.next().expect("unreachable");
                    if first_char == '+' || first_char == '-' {
                        integral_str = chars.as_str().to_owned();
                    }

                    integral_str = integral_str.trim_start_matches('0').to_owned();
                }

                let exponent_abs = exponent.abs() as usize;
                // Main idea here is to keep one of parts empty or zero exponent.
                if exponent >= 0 {
                    if exponent_abs >= fractional_str.len() {
                        integral_str.push_str(fractional_str.as_str());
                        exponent = exponent
                            .checked_sub(fractional_str.len() as i32) // `as i32`` is safe because `exponent.abs() as usize >= fractional_str.len()`
                            .ok_or(ConvertError::new("too small exponent"))?;
                        fractional_str = "".to_owned();
                    } else {
                        integral_str.push_str(&fractional_str[0..exponent_abs]);
                        fractional_str = fractional_str[exponent_abs..].to_owned();
                        exponent = 0;
                    }
                } else {
                    if exponent_abs >= integral_str.len() {
                        fractional_str.insert_str(0, integral_str.as_str());
                        exponent = exponent
                            .checked_add(integral_str.len() as i32) // `as i32`` is safe because `exponent.abs() as usize >= integral_str.len()`
                            .ok_or(ConvertError::new("too large exponent"))?;
                        integral_str = "".to_owned();
                    } else {
                        fractional_str.insert_str(
                            0,
                            &integral_str[integral_str.len() - exponent_abs..integral_str.len()],
                        );
                        integral_str = integral_str[..integral_str.len() - exponent_abs].to_owned();
                        exponent = 0;
                    }
                }

                if !integral_str.is_empty() {
                    integral_str = integral_str.trim_start_matches('0').to_owned();
                }

                // `exponent_abs` must be reevaluated
                let exponent_abs = exponent.abs() as usize;

                debug_assert!(
                    exponent == 0 || fractional_str.is_empty() || integral_str.is_empty()
                );
                let integral: $layout = if integral_str.is_empty() {
                    debug_assert!((exponent == 0 || fractional_str.is_empty()) && exponent <= 0);
                    0
                } else {
                    // If at this point `integral_str` part can't be represent as `$layout` then
                    // it obviously can't be represented as `$layout` after multiplication by `exponent`
                    // because `exponent` here is not less than zero.
                    debug_assert!((exponent == 0 || fractional_str.is_empty()) && exponent >= 0);
                    integral_str
                        .parse()
                        .map_err(|_| ConvertError::new("can't parse integral part"))?
                };

                if EXACT {
                    // if `fractional_str` contains trailing zeroes this error will be misleading
                    fractional_str = fractional_str.trim_end_matches('0').to_owned();
                    if fractional_str.len() > (Self::PRECISION.abs() + exponent) as usize {
                        return Err(ConvertError::new("requested precision is too high"));
                    }
                }

                let signum = if str.as_bytes()[0] == b'-' { -1 } else { 1 };
                let last_idx = Self::PRECISION + exponent;
                let last_idx_abs = last_idx.abs() as usize;
                let round = if !EXACT && last_idx >= 0 && last_idx_abs < fractional_str.len() {
                    let extra = fractional_str.as_bytes()[last_idx_abs];
                    fractional_str = fractional_str[..last_idx_abs].to_owned();
                    Some(signum).filter(|_| extra >= b'5')
                } else {
                    None
                };

                let ten: $layout = 10;
                let fractional_multiplier = ten.pow(
                    (fractional_str.len() + exponent_abs)
                        .try_into()
                        .map_err(|_| ConvertError::new("too big fractional_str"))?,
                );

                if EXACT && fractional_multiplier > Self::COEF {
                    return Err(ConvertError::new("requested precision is too high"));
                }

                debug_assert!(fractional_multiplier <= Self::COEF);

                let fractional: Option<$layout> = if !fractional_str.is_empty() {
                    Some(
                        fractional_str
                            .parse()
                            .map_err(|_| ConvertError::new("can't parse fractional part"))?,
                    )
                } else {
                    None
                };

                let integral_multiplier = ten.pow(
                    (Self::PRECISION + exponent)
                        .try_into()
                        .map_err(|_| ConvertError::new("too big exponent"))?,
                );
                let mut final_integral = integral
                    .checked_mul(integral_multiplier)
                    .ok_or(ConvertError::new("too big integral"))?;

                if signum < 0 {
                    final_integral = -final_integral;
                }

                let mut final_fractional = fractional
                    .map(|fractional| signum * Self::COEF / fractional_multiplier * fractional);

                if let Some(round) = round {
                    debug_assert!(!EXACT);
                    if let Some(final_fractional_inner) = final_fractional.as_mut() {
                        final_fractional = Some(
                            final_fractional_inner
                                .checked_add(round)
                                .ok_or(ConvertError::new("requested precision is too high"))?,
                        );
                    } else {
                        final_integral = final_integral
                            .checked_add(round)
                            .ok_or(ConvertError::new("too big integral"))?;
                    }
                }

                if let Some(&mut final_fractional) = final_fractional.as_mut() {
                    final_integral = final_integral
                        .checked_add(final_fractional)
                        .ok_or(ConvertError::new("too big number"))?;
                }

                Ok(Self::from_bits(final_integral))
            }
        }

        impl<P: Precision> Stringify for FixedPoint<$layout, P> {
            fn stringify(&self, buf: &mut StrBuf) {
                let mut fmt = itoa::Buffer::new();

                let sign = self.inner.signum();
                if sign < 0 {
                    let _ = buf.push('-');
                }

                let integral = (self.inner / Self::COEF).abs();
                let fractional = (self.inner % Self::COEF).abs();

                let _ = buf.push_str(fmt.format(integral));
                let _ = buf.push('.');

                if fractional > 0 {
                    let fractional_with_leading_one = fractional + Self::COEF;
                    let s = &fmt.format(fractional_with_leading_one)[1..];
                    let _ = buf.push_str(s.trim_end_matches('0'));
                } else {
                    let _ = buf.push('0');
                }
            }
        }
    };
}

// Serialize as a string in case of human readable formats.
// The maximum length can be calculated as `len(str(-2**bits)) + 1`,
// where `1` is reserved for `.` after integral part.
const MAX_LEN: usize = if cfg!(feature = "i128") { 41 } else { 21 };

// TODO: try `staticvec` after stabilization.
// Now it works faster than `arrayvec`.
#[allow(unreachable_pub)]
pub struct StrBuf {
    buffer: [u8; MAX_LEN],
    len: usize,
}

impl Default for StrBuf {
    fn default() -> Self {
        Self {
            buffer: [0; MAX_LEN],
            len: 0,
        }
    }
}

impl StrBuf {
    #[inline]
    fn push(&mut self, c: char) {
        debug_assert!(self.len < MAX_LEN);
        debug_assert!(c.is_ascii());

        unsafe { *self.buffer.as_mut().get_unchecked_mut(self.len) = c as u8 };
        self.len += 1;
    }

    #[inline]
    fn push_str(&mut self, s: &str) {
        debug_assert!(self.len + s.len() <= MAX_LEN);

        let s = s.as_bytes();
        let buf = unsafe { &mut self.buffer.get_unchecked_mut(self.len..self.len + s.len()) };
        buf.copy_from_slice(s);
        self.len += s.len();
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        unsafe {
            let buf = self.buffer.get_unchecked(..self.len);
            str::from_utf8_unchecked(buf)
        }
    }
}

// TODO: pass attrs to doc.
#[cfg(feature = "i16")]
impl_for!(i16);
#[cfg(feature = "i32")]
impl_for!(i32);
#[cfg(feature = "i64")]
impl_for!(i64);
#[cfg(feature = "i128")]
impl_for!(i128);
