use std::{
    fmt::{self, Display},
    io::{Cursor, Write as _},
    marker::PhantomData,
    str::{self, FromStr},
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::FixedPoint;

impl<I, P> Serialize for FixedPoint<I, P>
where
    I: Serialize,
    Self: Display,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            as_string::serialize(self, serializer)
        } else {
            as_repr::serialize(self, serializer)
        }
    }
}

impl<'de, I, P> Deserialize<'de> for FixedPoint<I, P>
where
    I: Deserialize<'de>,
    Self: FromStr,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            as_string::deserialize(deserializer)
        } else {
            as_repr::deserialize(deserializer)
        }
    }
}

/// (De)serializer `FixedPoint` as inner representation.
pub mod as_repr {
    use super::*;

    #[inline]
    pub fn serialize<I, P, S>(fp: &FixedPoint<I, P>, serializer: S) -> Result<S::Ok, S::Error>
    where
        I: Serialize,
        S: Serializer,
    {
        fp.inner.serialize(serializer)
    }

    #[inline]
    pub fn deserialize<'de, I, P, D>(deserializer: D) -> Result<FixedPoint<I, P>, D::Error>
    where
        I: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        I::deserialize(deserializer).map(FixedPoint::from_bits)
    }
}

/// (De)serializes `FixedPoint` as a string.
pub mod as_string {
    use super::*;

    pub fn serialize<I, P, S>(fp: &FixedPoint<I, P>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        FixedPoint<I, P>: Display,
    {
        // Serialize as a string in case of human readable formats.
        // The maximum length can be calculated as `len(str(-2**bits)) + 1`,
        // where `1` is reserved for `.` after integral part.
        const MAX_LEN: usize = if cfg!(feature = "i128") { 41 } else { 21 };

        let mut buf = [0; MAX_LEN];
        let mut cursor = Cursor::new(&mut buf[..]);
        let _ = write!(cursor, "{}", fp);
        let p = cursor.position() as usize;

        // The Display instance for numbers produces valid utf-8.
        let s = unsafe { str::from_utf8_unchecked(&buf[..p]) };

        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, I, P, D>(deserializer: D) -> Result<FixedPoint<I, P>, D::Error>
    where
        D: Deserializer<'de>,
        FixedPoint<I, P>: FromStr,
    {
        // Deserialize as a string in case of human readable formats.
        deserializer.deserialize_str(FixedPointVisitor::<I, P>(PhantomData))
    }

    struct FixedPointVisitor<I, P>(PhantomData<(I, P)>);

    impl<'de, I, P> de::Visitor<'de> for FixedPointVisitor<I, P>
    where
        FixedPoint<I, P>: FromStr,
    {
        type Value = FixedPoint<I, P>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a FixedPoint type representing a fixed-point number")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            // TODO: parse scientific form.
            // TODO: parse big ones with loss instead of an error.
            value
                .parse()
                .map_err(|_| E::invalid_value(de::Unexpected::Str(value), &self))
        }

        // TODO: visit_f64
    }
}
