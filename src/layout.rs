use core::convert::{From, TryInto};
use core::ops::{Add, Div, Mul, Neg, Sub};

#[doc(hidden)]
pub trait Promotion:
    Sized + Ord + Neg + Add + Sub + Mul + Div + From<Self::Layout> + TryInto<Self::Layout>
{
    type Layout;

    fn as_layout(&self) -> Self::Layout;
    #[cfg(feature = "std")]
    fn as_positive_f64(&self) -> f64;
    fn leading_zeros(&self) -> u32;
    fn mul_l(&self, rhs: Self::Layout) -> Self;
    fn div_l(&self, rhs: Self::Layout) -> Self;
    fn div_rem_l(&self, rhs: Self::Layout) -> (Self, Self::Layout);
}

#[cfg(any(feature = "i16", feature = "i32", feature = "i64"))]
macro_rules! promotion {
    ($layout:ty => $prom:ty) => {
        impl Promotion for $prom {
            type Layout = $layout;

            #[inline]
            fn as_layout(&self) -> Self::Layout {
                *self as $layout
            }

            #[cfg(feature = "std")]
            #[inline]
            fn as_positive_f64(&self) -> f64 {
                *self as f64
            }

            #[inline]
            fn leading_zeros(&self) -> u32 {
                (*self).leading_zeros()
            }

            #[inline]
            fn mul_l(&self, rhs: Self::Layout) -> Self {
                self * rhs as $prom
            }

            #[inline]
            fn div_l(&self, rhs: Self::Layout) -> Self {
                self / rhs as $prom
            }

            #[inline]
            fn div_rem_l(&self, rhs: Self::Layout) -> (Self, Self::Layout) {
                (self / rhs as $prom, (self % rhs as $prom) as Self::Layout)
            }
        }
    };
}

#[cfg(feature = "i16")]
promotion!(i16 => i32);
#[cfg(feature = "i32")]
promotion!(i32 => i64);
#[cfg(feature = "i64")]
promotion!(i64 => i128);
// NOTE: i128 => i256 is implemented in the `i256_polyfill` module.
