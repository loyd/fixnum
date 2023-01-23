use borsh::{
    maybestd::io::{Read, Result, Write},
    BorshDeserialize, BorshSerialize,
};

use crate::FixedPoint;

macro_rules! impl_borsh {
    ($layout:ty, $(#[$attr:meta])?) => {
        #[cfg_attr(docsrs, doc(cfg(feature = "borsh")))]
        $(#[$attr])?
        impl<P> BorshSerialize for FixedPoint<$layout, P> {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                self.as_bits().serialize(writer)
            }
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "borsh")))]
        $(#[$attr])?
        impl<P> BorshDeserialize for FixedPoint<$layout, P> {
            #[inline]
            fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
                <$layout>::deserialize_reader(reader).map(Self::from_bits)
            }
        }
    };
}

#[cfg(feature = "i16")]
impl_borsh!(i16, #[cfg_attr(docsrs, doc(cfg(feature = "i16")))]);
#[cfg(feature = "i32")]
impl_borsh!(i32, #[cfg_attr(docsrs, doc(cfg(feature = "i32")))]);
#[cfg(feature = "i64")]
impl_borsh!(i64, #[cfg_attr(docsrs, doc(cfg(feature = "i64")))]);
#[cfg(feature = "i128")]
impl_borsh!(i128, #[cfg_attr(docsrs, doc(cfg(feature = "i128")))]);

#[cfg(all(test, any(feature = "i64", feature = "i128")))]
mod test {
    use core::{fmt::Debug, str::FromStr};

    use borsh::maybestd::vec::Vec;

    use super::*;

    fn roundtrip<T>(values: &[&str])
    where
        T: Debug + FromStr + PartialEq + BorshDeserialize + BorshSerialize,
        <T as FromStr>::Err: Debug,
    {
        #[derive(Debug, PartialEq, BorshDeserialize, BorshSerialize)]
        struct FooBar<T> {
            a: T,
            b: Vec<T>,
        }

        let b = values.iter().map(|s| s.parse().unwrap()).collect();
        let x = FooBar {
            a: "3163.422490387".parse().unwrap(),
            b,
        };
        let bytes = x.try_to_vec().unwrap();
        assert_eq!(FooBar::<T>::try_from_slice(&bytes).unwrap(), x);
    }

    #[test]
    #[cfg(feature = "i64")]
    fn roundtrip_i64() {
        roundtrip::<FixedPoint<i64, typenum::U9>>(&[
            "-11243.992874173",
            "11243.992874173",
            "0",
            "-72324",
        ]);
    }

    #[test]
    #[cfg(feature = "i128")]
    fn roundtrip_i128() {
        roundtrip::<FixedPoint<i128, typenum::U18>>(&[
            "-11243.992874173992874173",
            "11243.992992874173874173",
            "0",
            "-7232432454934",
        ]);
    }
}
