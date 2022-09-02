use crate::{
    ops::Zero,
    power_table::{
        power_of_10, rdiv_by_exponent_10, MAX_EXPONENT_5, NEXT_EXPONENT_10, POWERS_OF_10,
        POWERS_OF_5,
    },
    ConvertError, FixedPoint, Precision,
};

macro_rules! impl_try_from_f64 {
    ($layout:tt) => {
        impl<P: Precision> TryFrom<f64> for FixedPoint<$layout, P> {
            type Error = ConvertError;

            /// Implementation courtesy of [`rust_decimal` crate][rust_decimal]
            ///
            /// [rust_decimal]: https://github.com/paupino/rust-decimal/blob/2de2a6dd2f385e98c4019ebe38b5c6de5fef6cba/src/decimal.rs#L2059
            fn try_from(value: f64) -> Result<Self, Self::Error> {
                if !value.is_finite() {
                    return Err(ConvertError::new("not finite"));
                }

                // f64 is being broken up by bits i.e. 1/11/52 (sign, biased_exponent, mantissa)
                // See https://en.wikipedia.org/wiki/IEEE_754-1985
                // n = (-1)^sign * 2^exp * significand
                // fixnum stores it differently: n = significand * 10^(-PRECISION)
                let raw = value.to_bits();
                let positive = (raw >> 63) == 0;
                let biased_exponent = ((raw >> 52) & 0x7FF) as i32;
                let mut bits = (raw & 0x000F_FFFF_FFFF_FFFF) as u128;

                // Handle the special zero case
                if biased_exponent == 0 && bits == 0 {
                    return Ok(Self::ZERO);
                }

                // Get the bits and exponent2
                let mut exponent2 = if biased_exponent == 0 {
                    // Denormalized number
                    -1022
                } else {
                    // Add extra hidden bit to mantissa
                    bits |= 0x0010_0000_0000_0000;
                    biased_exponent - 1023
                };

                // The act of copying a significand as integer bits is equivalent to shifting
                // left the significand 52 bits. The exponent is reduced to compensate.
                exponent2 -= 52;

                // 2^exponent2 = 10^exponent2 / 5^exponent2 =
                //             = 10^exponent2 * 5^(-exponent2)
                let mut exponent5 = -exponent2;
                let mut exponent10 = exponent2; // Ultimately, we want this for the scale

                if exponent5 > 0 {
                    // Divide significand by 2 as much as possible without losing precision
                    let excess_exponent2 = bits.trailing_zeros().min(exponent5 as u32) as i32;
                    exponent10 += excess_exponent2;
                    exponent5 -= excess_exponent2;
                    bits >>= excess_exponent2;

                    if exponent5 > 0 {
                        // The significand is no more divisible by 2. Therefore the significand should
                        // be multiplied by 5, unless the multiplication overflows.
                        let lz = bits.leading_zeros() as usize;
                        let reduced_exponent5 = if lz == 0 {
                            0
                        } else {
                            let multiplier_exponent5 = exponent5.min(MAX_EXPONENT_5[lz - 1] as i32);
                            bits *= POWERS_OF_5[multiplier_exponent5 as usize];
                            if let (true, Some(b)) =
                                (multiplier_exponent5 < exponent5, bits.checked_mul(5))
                            {
                                bits = b;
                                multiplier_exponent5 + 1
                            } else {
                                multiplier_exponent5
                            }
                        };

                        if reduced_exponent5 == 0 {
                            // Multiplication by 5 overflows. The significand should be divided
                            // by 2, and therefore will lose significant digits.
                            exponent10 += 1;
                            exponent5 -= 1;
                            bits >>= 1;
                        } else {
                            exponent5 -= reduced_exponent5;
                        }

                        while exponent5 > 0 {
                            if bits & 1 == 0 {
                                exponent10 += 1;
                                exponent5 -= 1;
                                bits >>= 1;
                            } else {
                                if let Some(b) = bits.checked_mul(5) {
                                    exponent5 -= 1;
                                    bits = b;
                                } else {
                                    // Multiplication by 5 overflows. The significand should be divided
                                    // by 2, and therefore will lose significant digits.
                                    exponent10 += 1;
                                    exponent5 -= 1;
                                    bits >>= 1;
                                }
                            }
                        }
                    }
                }

                // In order to divide the value by 5, it is best to multiply by 2/10.
                // Therefore, exponent10 is decremented, and the significand should be multiplied by 2.
                while exponent5 < 0 {
                    const MOST_SIGNIFICANT_BIT: u128 = !(u128::MAX >> 1);
                    bits = if bits & MOST_SIGNIFICANT_BIT == 0 {
                        // No far left bit, the significand can withstand a shift-left without overflowing
                        exponent10 -= 1;
                        exponent5 += 1;
                        bits << 1
                    } else {
                        // The significand would overflow if shifted. Therefore it should be
                        // directly divided by 5. This will lose significant digits, unless
                        // by chance the significand happens to be divisible by 5.
                        exponent5 += 1;
                        bits / 5
                    };
                }

                // At this point, the significand has assimilated the exponent5

                // This step is required in order to remove excess bits of precision from the
                // end of the bit representation, down to the precision guaranteed by the
                // floating point number
                // Guaranteed to about 16 dp
                let prefix = bits >> 52;
                if exponent10 < 0 && prefix > 0 {
                    let lz = (bits.leading_zeros() + 52) as usize;
                    let mut divisor_exponent_10 = NEXT_EXPONENT_10[lz] as i32;
                    let divisor = power_of_10(divisor_exponent_10 as u32).unwrap();
                    if prefix >= divisor {
                        divisor_exponent_10 = NEXT_EXPONENT_10[lz - 1] as i32;
                    }
                    let divisor_exponent_10 = divisor_exponent_10.min(-exponent10 as i32);
                    let (divisor, remainder) = POWERS_OF_10[divisor_exponent_10 as usize];
                    let res = bits / divisor;
                    bits = if bits % divisor > remainder {
                        res + 1
                    } else {
                        res
                    };
                    exponent10 += divisor_exponent_10;
                }

                // exponent10 must equal to -PRECISION, so the significand must be scaled up or down appropriately.
                if exponent10 > -Self::PRECISION {
                    // In order to bring exponent10 down, the significand should be
                    // multiplied by 10 to compensate. If the exponent10 is too big, this
                    // will cause the significand to overflow.
                    bits = power_of_10((exponent10 + Self::PRECISION) as u32)
                        .and_then(|multiplier| bits.checked_mul(multiplier))
                        .ok_or_else(|| ConvertError::new("too big number"))?;
                } else if exponent10 < -Self::PRECISION {
                    // In order to bring exponent up to -PRECISION, the significand should
                    // be divided by 10 to compensate. If the exponent10 is too small, this
                    // will cause the significand to underflow and become 0.
                    bits = rdiv_by_exponent_10(bits, (-Self::PRECISION - exponent10) as u32);
                }

                let bits: $layout = bits
                    .try_into()
                    .map_err(|_| ConvertError::new("too big number"))?;

                if positive {
                    Ok(Self::from_bits(bits))
                } else {
                    bits.checked_neg()
                        .map(Self::from_bits)
                        .ok_or_else(|| ConvertError::new("too big number"))
                }
            }
        }
    };
}

// TODO: pass attrs to doc.
#[cfg(feature = "i16")]
impl_try_from_f64!(i16);
#[cfg(feature = "i32")]
impl_try_from_f64!(i32);
#[cfg(feature = "i64")]
impl_try_from_f64!(i64);
#[cfg(feature = "i128")]
impl_try_from_f64!(i128);
