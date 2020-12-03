use crate::FixedPoint;

// TODO: make it Sealed
pub trait Operand<R> {
    type Promotion;
    fn promote(self) -> Self::Promotion;
}

// TODO: restrict `I` and `P`.
impl<I, P> Operand<FixedPoint<I, P>> for FixedPoint<I, P> {
    type Promotion = FixedPoint<I, P>;
    #[inline]
    fn promote(self) -> Self::Promotion {
        self
    }
}

macro_rules! impl_int_operand {
    ($int:ty => $($to:ty),*) => {
        $(
            impl Operand<$to> for $int {
                type Promotion = $to;
                #[inline]
                fn promote(self) -> Self::Promotion {
                    self.into()
                }
            }
        )*

        impl<I, P> Operand<FixedPoint<I, P>> for $int
            where $int: Operand<I>
        {
            type Promotion = <$int as Operand<I>>::Promotion;
            #[inline]
            fn promote(self) -> Self::Promotion {
                Operand::<I>::promote(self)
            }
        }

    }
}

// TODO: unsigned?
impl_int_operand!(i8 => i8, i16, i32, i64, i128);
impl_int_operand!(i16 => i16, i32, i64, i128);
impl_int_operand!(i32 => i32, i64, i128);
impl_int_operand!(i64 => i64, i128);
impl_int_operand!(i128 => i128);

#[macro_export]
macro_rules! legit_op {
    ($lhs:ty [cadd] $rhs:ty = $res:tt) => {
        impl $crate::ops::CheckedAdd<$rhs> for $lhs {
            type Output = $res;
            type Error = $crate::ArithmeticError;

            #[inline]
            fn cadd(self, rhs: $rhs) -> Result<$res, $crate::ArithmeticError> {
                crate::legit_op!(@method (l = self, r = rhs) => l.cadd(r), $res)
            }
        }
    };
    ($lhs:ty [csub] $rhs:ty = $res:tt) => {
        impl $crate::ops::CheckedSub<$rhs> for $lhs {
            type Output = $res;
            type Error = $crate::ArithmeticError;

            #[inline]
            fn csub(self, rhs: $rhs) -> Result<$res, $crate::ArithmeticError> {
                crate::legit_op!(@method (l = self, r = rhs) => l.csub(r), $res)
            }
        }
    };
    ($lhs:ty [cmul] $rhs:ty = $res:tt) => {
        impl $crate::ops::CheckedMul<$rhs> for $lhs {
            type Output = $res;
            type Error = $crate::ArithmeticError;

            #[inline]
            fn cmul(self, rhs: $rhs) -> Result<$res, $crate::ArithmeticError> {
                crate::legit_op!(@method (l = self, r = rhs) => l.cmul(r), $res)
            }
        }
    };
    ($lhs:ty [rmul] $rhs:ty = $res:tt) => {
        impl $crate::ops::RoundingMul<$rhs> for $lhs {
            type Output = $res;
            type Error = $crate::ArithmeticError;

            #[inline]
            fn rmul(
                self,
                rhs: $rhs,
                mode: $crate::ops::RoundMode,
            ) -> Result<$res, $crate::ArithmeticError> {
                crate::legit_op!(@method (l = self, r = rhs) => l.rmul(r, mode), $res)
            }
        }
    };
    ($lhs:ty [rdiv] $rhs:ty = $res:tt) => {
        impl $crate::ops::RoundingDiv<$rhs> for $lhs {
            type Output = $res;
            type Error = $crate::ArithmeticError;

            #[inline]
            fn rdiv(
                self,
                rhs: $rhs,
                mode: $crate::ops::RoundMode,
            ) -> Result<$res, $crate::ArithmeticError> {
                use core::convert::TryInto;
                crate::legit_op!(@method (l = self, r = rhs) => {
                    $res(
                        l.try_into().map_err(|_| $crate::ArithmeticError::Overflow)?
                    ).0.rdiv(r, mode)
                }, $res)
            }
        }
    };
    (@method ($l:ident = $lhs:expr, $r:ident = $rhs:expr) => $op:expr, $res:tt) => {{
        use $crate::_priv::*;
        fn up<I, O: Operand<I>>(operand: O, _: impl FnOnce(I) -> $res) -> O::Promotion {
            operand.promote()
        }
        let $l = up($lhs.0, $res);
        let $r = up($rhs.0, $res);
        $op.map($res)
    }};
}
