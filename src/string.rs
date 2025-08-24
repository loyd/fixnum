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

            /// Parses a fixed-point number without scientific notation.
            ///
            /// Note: the input `str` must be already trimmed (no leading/trailing whitespace).
            /// Trimming is performed by the caller (`parse_str_with_scientific`).
            fn parse_str_without_scientific<const EXACT: bool>(
                str: &str,
            ) -> Result<Self, ConvertError> {
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
                    if fractional_str.len() > Self::PRECISION.unsigned_abs() as usize {
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
                let trimmed_input = str.trim();

                // Fast path: no scientific notation
                let (mantissa_str, exponent_str) = if let Some(exponent_char) =
                    trimmed_input.chars().find(|c| *c == 'e' || *c == 'E')
                {
                    if let Some(parts) = trimmed_input.split_once(exponent_char) {
                        parts
                    } else {
                        return Err(ConvertError::new(
                            "unable to split string by exponent char",
                        ));
                    }
                } else {
                    return Self::parse_str_without_scientific::<EXACT>(trimmed_input);
                };

                let mut exponent: i32 = exponent_str
                    .parse::<i32>()
                    .map_err(|_| ConvertError::new("can't parse exponent"))?;

                // Work with mantissa: optional sign, optional dot
                let mut mantissa_bytes = mantissa_str.as_bytes();
                if !mantissa_bytes.is_empty()
                    && (mantissa_bytes[0] == b'+' || mantissa_bytes[0] == b'-')
                {
                    mantissa_bytes = &mantissa_bytes[1..];
                }

                // Split on decimal point
                let (integral_part_digits, fractional_part_digits) =
                    match mantissa_bytes.iter().position(|&b| b == b'.') {
                        Some(p) => (&mantissa_bytes[..p], &mantissa_bytes[p + 1..]),
                        None => (mantissa_bytes, &mantissa_bytes[0..0]),
                };

                // Trim integral leading zeros
                let integral_trim_offset = integral_part_digits
                    .iter()
                    .position(|&b| b != b'0')
                    .unwrap_or(integral_part_digits.len());
                let mut integral_primary_digits = &integral_part_digits[integral_trim_offset..];
                let mut integral_from_fractional_digits: &[u8] = &[]; // moved from fractional when exponent >= 0

                let mut fractional_primary_digits: &[u8] = fractional_part_digits; // primary fractional segment
                let mut fractional_from_integral_digits: &[u8] = &[]; // from integral when exponent < 0

                // Move digits according to exponent without allocations
                if exponent >= 0 {
                    let digits_to_move_count = (exponent as usize).min(fractional_primary_digits.len());
                    integral_from_fractional_digits = &fractional_primary_digits[..digits_to_move_count];
                    fractional_primary_digits = &fractional_primary_digits[digits_to_move_count..];
                    exponent -= digits_to_move_count as i32;
                } else {
                    let digits_needed_from_integral = (-exponent) as usize;
                    if digits_needed_from_integral >= integral_primary_digits.len() {
                        // Move entire integral into fractional
                        fractional_from_integral_digits = fractional_primary_digits;
                        fractional_primary_digits = integral_primary_digits;
                        integral_primary_digits = &[];
                        exponent += digits_needed_from_integral as i32; // == +integral_primary_digits_len (old)
                    } else {
                        // Split integral; tail goes to fractional front
                        let split_index = integral_primary_digits.len() - digits_needed_from_integral;
                        fractional_from_integral_digits = fractional_primary_digits;
                        fractional_primary_digits = &integral_primary_digits[split_index..];
                        integral_primary_digits = &integral_primary_digits[..split_index];
                        exponent = 0;
                    }
                }

                // Trim integral leading zeros again (may appear after moving)
                if !integral_primary_digits.is_empty() {
                    let trim_offset = integral_primary_digits
                        .iter()
                        .position(|&b| b != b'0')
                        .unwrap_or(integral_primary_digits.len());
                    integral_primary_digits = &integral_primary_digits[trim_offset..];
                }
                if integral_primary_digits.is_empty() && !integral_from_fractional_digits.is_empty() {
                    let trim_offset = integral_from_fractional_digits
                        .iter()
                        .position(|&b| b != b'0')
                        .unwrap_or(integral_from_fractional_digits.len());
                    integral_from_fractional_digits = &integral_from_fractional_digits[trim_offset..];
                }

                // Helpers over two-slice sequences
                #[inline]
                fn sequence_len(a: &[u8], b: &[u8]) -> usize {
                    a.len() + b.len()
                }
                #[inline]
                fn get_digit_at_index(a: &[u8], b: &[u8], i: usize) -> Option<u8> {
                    if i < a.len() {
                        Some(a[i])
                    } else {
                        let j = i - a.len();
                        if j < b.len() { Some(b[j]) } else { None }
                    }
                }
                #[inline]
                fn trim_fractional_trailing_zeros<'a>(
                    mut a: &'a [u8],
                    mut b: &'a [u8],
                ) -> (&'a [u8], &'a [u8]) {
                    if !b.is_empty() {
                        let mut end = b.len();
                        while end > 0 && b[end - 1] == b'0' {
                            end -= 1;
                        }
                        b = &b[..end];
                    }
                    if b.is_empty() && !a.is_empty() {
                        let mut end = a.len();
                        while end > 0 && a[end - 1] == b'0' {
                            end -= 1;
                        }
                        a = &a[..end];
                    }
                    (a, b)
                }
                

                // Re-evaluate absolute exponent after shifts
                let exponent_abs_usize = exponent.abs() as usize;

                // Parse integral part (across two segments) or zero
                let ten: $layout = 10;
                let mut integral_value: $layout = 0;
                if !integral_primary_digits.is_empty() || !integral_from_fractional_digits.is_empty() {
                    // Ensure digits-only and accumulate
                    for &seg in &[integral_primary_digits, integral_from_fractional_digits] {
                        for &b in seg {
                            if !(b'0'..=b'9').contains(&b) {
                                return Err(ConvertError::new(
                                    "can't parse integral part: must contain digits only",
                                ));
                            }
                            let d = (b - b'0') as $layout;
                            integral_value = integral_value
                                .checked_mul(ten)
                                .and_then(|v| v.checked_add(d))
                                .ok_or(ConvertError::new("overflow: integral part"))?;
                        }
                    }
                }

                if EXACT {
                    // If `fractional_str` contains trailing zeroes this error will be misleading
                    let (fa, fb) = trim_fractional_trailing_zeros(fractional_primary_digits, fractional_from_integral_digits);
                    fractional_primary_digits = fa;
                    fractional_from_integral_digits = fb;
                    if sequence_len(fractional_primary_digits, fractional_from_integral_digits)
                        > (Self::PRECISION.abs() + exponent) as usize
                    {
                        return Err(ConvertError::new("out of range: precision exceeds scale"));
                    }
                }

                let signum: $layout = if trimmed_input.starts_with('-') { -1 } else { 1 };
                let rounding_index = Self::PRECISION + exponent;
                let rounding_index_abs = rounding_index.abs() as usize;

                // Determine rounding digit and effective fractional length
                let mut round_adjustment: Option<$layout> = None;
                let mut effective_fractional_primary = fractional_primary_digits;
                let mut effective_fractional_secondary = fractional_from_integral_digits;
                let fractional_total_len = sequence_len(fractional_primary_digits, fractional_from_integral_digits);
                if !EXACT && rounding_index >= 0 && rounding_index_abs < fractional_total_len {
                    if let Some(extra) = get_digit_at_index(
                        fractional_primary_digits,
                        fractional_from_integral_digits,
                        rounding_index_abs,
                    ) {
                        round_adjustment = Some(signum).filter(|_| extra >= b'5');
                    }
                    // Truncate fractional to first `last_idx_abs` digits
                    if rounding_index_abs <= fractional_primary_digits.len() {
                        effective_fractional_primary = &fractional_primary_digits[..rounding_index_abs];
                        effective_fractional_secondary = &[];
                    } else {
                        effective_fractional_primary = fractional_primary_digits;
                        effective_fractional_secondary =
                            &fractional_from_integral_digits[..(rounding_index_abs - fractional_primary_digits.len())];
                    }
                }

                // Fractional multiplier 10^(len(frac) + |exponent|)
                let effective_fractional_len = sequence_len(effective_fractional_primary, effective_fractional_secondary);
                let fractional_multiplier_digits: usize = effective_fractional_len + exponent_abs_usize;
                let fractional_multiplier = ten.pow(
                    u32::try_from(fractional_multiplier_digits)
                        .map_err(|_| ConvertError::new("out of range: precision exceeds scale"))?,
                );

                if EXACT && fractional_multiplier > Self::COEF {
                    return Err(ConvertError::new("out of range: precision exceeds scale"));
                }

                debug_assert!(fractional_multiplier <= Self::COEF);

                // Parse fractional digits (if any)
                let mut fractional_value_opt: Option<$layout> = None;
                if effective_fractional_len > 0 {
                    let mut acc: $layout = 0;
                    for &seg in &[effective_fractional_primary, effective_fractional_secondary] {
                        for &b in seg {
                            if !(b'0'..=b'9').contains(&b) {
                                return Err(ConvertError::new(
                                    "can't parse fractional part: must contain digits only",
                                ));
                            }
                            let d = (b - b'0') as $layout;
                            acc = acc
                                .checked_mul(ten)
                                .and_then(|v| v.checked_add(d))
                                .ok_or(ConvertError::new("overflow: fractional part"))?;
                        }
                    }
                    fractional_value_opt = Some(acc);
                }

                // Integral multiplier 10^(PRECISION + exponent)
                let integral_multiplier = ten.pow(
                    (Self::PRECISION + exponent)
                        .try_into()
                        .map_err(|_| ConvertError::new("out of range: exponent"))?,
                );

                let mut final_integral_value = integral_value
                    .checked_mul(integral_multiplier)
                    .ok_or(ConvertError::new("overflow: integral part"))?;

                if signum < 0 {
                    final_integral_value = -final_integral_value;
                }

                let mut final_fractional_value_opt = fractional_value_opt
                    .map(|f| signum * Self::COEF / fractional_multiplier * f);

                if let Some(round_adjust) = round_adjustment {
                    debug_assert!(!EXACT);
                    if let Some(ref mut final_fractional_value) = final_fractional_value_opt {
                        *final_fractional_value = final_fractional_value
                            .checked_add(round_adjust)
                            .ok_or(ConvertError::new("out of range: precision exceeds scale"))?;
                    } else {
                        final_integral_value = final_integral_value
                            .checked_add(round_adjust)
                            .ok_or(ConvertError::new("overflow: integral part"))?;
                    }
                }

                if let Some(final_fractional_value) = final_fractional_value_opt {
                    final_integral_value = final_integral_value
                        .checked_add(final_fractional_value)
                        .ok_or(ConvertError::new("overflow: result"))?;
                }

                Ok(Self::from_bits(final_integral_value))
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
