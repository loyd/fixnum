use crate::{layout::Promotion, ops::Zero};

pub(crate) trait Sqrt: Promotion {
    fn sqrt(self) -> Self::Layout;
}

macro_rules! impl_sqrt {
    ($prom:ty) => {
        impl Sqrt for $prom {
            /// Checked integer square root.
            /// Sqrt implementation courtesy of [`num` crate][num].
            ///
            /// [num]: https://github.com/rust-num/num-integer/blob/4d166cbb754244760e28ea4ce826d54fafd3e629/src/roots.rs#L278
            #[inline]
            fn sqrt(self) -> Self::Layout {
                type Layout = <$prom as Promotion>::Layout;

                #[cfg(feature = "std")]
                #[inline]
                fn guess(v: $prom) -> Layout {
                    v.as_positive_f64().sqrt() as Layout
                }

                #[cfg(not(feature = "std"))]
                #[inline]
                fn guess(v: $prom) -> Layout {
                    #[inline]
                    fn log2_estimate(v: $prom) -> u32 {
                        debug_assert!(v > <$prom as Zero>::ZERO);
                        (core::mem::size_of::<$prom>() as u32 * 8) - 1 - v.leading_zeros()
                    }

                    1 << ((log2_estimate(v) + 1) / 2)
                }

                #[inline]
                fn fixpoint(mut x: Layout, f: impl Fn(Layout) -> Layout) -> Layout {
                    let mut xn = f(x);
                    while x < xn {
                        x = xn;
                        xn = f(x);
                    }
                    while x > xn {
                        x = xn;
                        xn = f(x);
                    }
                    x
                }

                debug_assert!(self >= <$prom as Zero>::ZERO);

                if self < <$prom>::from(4i8) {
                    return ((self > <$prom as Zero>::ZERO) as i8).into();
                }

                // https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method
                let next = |x: Layout| (self.div_l(x).as_layout() + x) >> 1;
                fixpoint(guess(self), next)
            }
        }
    };
}

#[cfg(feature = "i16")]
impl_sqrt!(i32);
#[cfg(feature = "i32")]
impl_sqrt!(i64);
#[cfg(feature = "i64")]
impl_sqrt!(i128);
#[cfg(feature = "i128")]
impl_sqrt!(crate::i256);
