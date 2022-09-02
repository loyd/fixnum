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

            fn from_str(str: &str) -> Result<Self, Self::Err> {
                let str = str.trim();

                let (integral_str, fractional_str) = if let Some(parts) = str.split_once('.') {
                    parts
                } else {
                    return str
                        .parse::<$layout>()
                        .map_err(|_| ConvertError::new("can't parse integral part of the str"))?
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

                if fractional_str.len() > Self::PRECISION.abs() as usize {
                    return Err(ConvertError::new("requested precision is too high"));
                }

                let ten: $layout = 10;
                let exp = ten.pow(fractional_str.len() as u32);

                if exp > Self::COEF {
                    return Err(ConvertError::new("requested precision is too high"));
                }

                let fractional: $layout = fractional_str
                    .parse()
                    .map_err(|_| ConvertError::new("can't parse fractional part"))?;

                let final_integral = integral
                    .checked_mul(Self::COEF)
                    .ok_or(ConvertError::new("too big integral"))?;
                let signum = if str.as_bytes()[0] == b'-' { -1 } else { 1 };
                let final_fractional = signum * Self::COEF / exp * fractional;

                final_integral
                    .checked_add(final_fractional)
                    .map(Self::from_bits)
                    .ok_or(ConvertError::new("too big number"))
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
