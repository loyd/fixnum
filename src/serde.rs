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
            // Serialize as a string in case of human readable formats.
            // The maximum length can be calculated as `len(str(-2**bits)) + 1`,
            // where `1` is reserved for `.` after integral part.
            const MAX_LEN: usize = if cfg!(feature = "i128") { 41 } else { 21 };

            let mut buf = [0; MAX_LEN];
            let mut cursor = Cursor::new(&mut buf[..]);
            let _ = write!(cursor, "{}", self);
            let p = cursor.position() as usize;

            // The Display instance for numbers produces valid utf-8.
            let s = unsafe { str::from_utf8_unchecked(&buf[..p]) };

            serializer.serialize_str(s)
        } else {
            self.inner.serialize(serializer)
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
            // Deserialize as a string in case of human readable formats.
            deserializer.deserialize_str(FixedPointVisitor::<I, P>(PhantomData))
        } else {
            I::deserialize(deserializer).map(FixedPoint::from_bits)
        }
    }
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
