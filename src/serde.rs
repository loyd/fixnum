//! A module that contains instances of `Serialize` and `Deserialize` for `FixedPoint`.
//! Also contains submodule that can be provided to `serde(with)` in order to
//! change the implementation.
//!
//! By default, `FixedPoint` is serialized using `str` for human readable formats
//! and `repr` for binary ones.
//!
//! By default, `FixedPoint` is deserialized from strings, floats and integers for human readable
//! formats and `repr` for binary ones.

use core::{fmt, marker::PhantomData, str::FromStr};

use serde::{
    de::{self, Error as _},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{string::Stringify, FixedPoint};

impl<I, P> Serialize for FixedPoint<I, P>
where
    I: Serialize,
    Self: Stringify + Clone,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            str::serialize(self, serializer)
        } else {
            repr::serialize(self, serializer)
        }
    }
}

impl<'de, I, P> Deserialize<'de> for FixedPoint<I, P>
where
    I: Deserialize<'de>,
    Self: FromStr + TryFrom<f64> + TryFrom<u64> + TryFrom<i64> + TryFrom<i128> + TryFrom<u128>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(FixedPointVisitor(PhantomData))
        } else {
            repr::deserialize(deserializer)
        }
    }
}

struct FixedPointVisitor<I, P>(PhantomData<(I, P)>);

impl<'de, I, P> de::Visitor<'de> for FixedPointVisitor<I, P>
where
    FixedPoint<I, P>:
        FromStr + TryFrom<f64> + TryFrom<u64> + TryFrom<i64> + TryFrom<i128> + TryFrom<u128>,
{
    type Value = FixedPoint<I, P>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("number or string containing a fixed-point number")
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        s.parse()
            .map_err(|_| E::invalid_value(de::Unexpected::Str(s), &self))
    }

    fn visit_f64<E: de::Error>(self, f: f64) -> Result<Self::Value, E> {
        Self::Value::try_from(f).map_err(|_| E::invalid_value(de::Unexpected::Float(f), &self))
    }

    fn visit_i64<E: de::Error>(self, i: i64) -> Result<Self::Value, E> {
        Self::Value::try_from(i).map_err(|_| E::invalid_value(de::Unexpected::Signed(i), &self))
    }

    fn visit_u64<E: de::Error>(self, u: u64) -> Result<Self::Value, E> {
        Self::Value::try_from(u).map_err(|_| E::invalid_value(de::Unexpected::Unsigned(u), &self))
    }

    fn visit_i128<E: de::Error>(self, i: i128) -> Result<Self::Value, E> {
        Self::Value::try_from(i).map_err(|_| {
            E::invalid_value(
                if i as i64 as i128 == i {
                    de::Unexpected::Signed(i as i64)
                } else {
                    de::Unexpected::Other("big i128")
                },
                &self,
            )
        })
    }

    fn visit_u128<E: de::Error>(self, u: u128) -> Result<Self::Value, E> {
        Self::Value::try_from(u).map_err(|_| {
            E::invalid_value(
                if u as u64 as u128 == u {
                    de::Unexpected::Unsigned(u as u64)
                } else {
                    de::Unexpected::Other("big u128")
                },
                &self,
            )
        })
    }

    // Support for `quick-xml` tags: `<tag>42.42</tag>`
    #[cfg(feature = "quick-xml")]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        // We must use `String` here.
        // Otherwise, `invalid type: string \"$value\", expected a borrowed string`.
        let key = map
            .next_key::<String>()
            .map_err(|_| A::Error::invalid_type(de::Unexpected::Map, &self))?;

        if key.as_deref() != Some("$value") {
            return Err(A::Error::invalid_type(de::Unexpected::Map, &self));
        }

        // We use `String` here to support `quick-xml v0.22`. In an actual one it's already fixed.
        let value = map
            .next_value::<String>()
            .map_err(|_| A::Error::invalid_type(de::Unexpected::Map, &self))?;

        value
            .parse()
            .map_err(|_| A::Error::invalid_value(de::Unexpected::Str(&value), &self))
    }

    // TODO: support serde_json/arbitrary_precision.
}

/// (De)serializes `FixedPoint` as inner representation.
pub mod repr {
    use super::*;

    /// Serializes to inner representation.
    #[inline]
    pub fn serialize<F, I, P, S>(fp: &F, serializer: S) -> Result<S::Ok, S::Error>
    where
        F: Into<FixedPoint<I, P>> + Clone,
        I: Serialize,
        S: Serializer,
    {
        fp.clone().into().into_bits().serialize(serializer)
    }

    /// Deserializes from inner representation.
    #[inline]
    pub fn deserialize<'de, F, I, P, D>(deserializer: D) -> Result<F, D::Error>
    where
        F: From<FixedPoint<I, P>>,
        I: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        I::deserialize(deserializer)
            .map(FixedPoint::from_bits)
            .map(F::from)
    }
}

/// (De)serializes `Option<FixedPoint>` as inner representation.
pub mod repr_option {
    use super::*;

    /// Serializes to inner representation.
    #[inline]
    pub fn serialize<F, I, P, S>(fp: &Option<F>, serializer: S) -> Result<S::Ok, S::Error>
    where
        F: Into<FixedPoint<I, P>> + Clone,
        I: Serialize,
        S: Serializer,
    {
        fp.clone()
            .map(Into::into)
            .map(FixedPoint::into_bits)
            .serialize(serializer)
    }

    /// Deserializes from inner representation.
    #[inline]
    pub fn deserialize<'de, F, I, P, D>(deserializer: D) -> Result<Option<F>, D::Error>
    where
        F: From<FixedPoint<I, P>>,
        I: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Option::<I>::deserialize(deserializer)
            .map(|inner| inner.map(FixedPoint::from_bits).map(F::from))
    }
}

/// (De)serializes `FixedPoint` as a string.
pub mod str {
    use super::*;

    /// Serializes to a string.
    pub fn serialize<F, I, P, S>(fp: &F, serializer: S) -> Result<S::Ok, S::Error>
    where
        F: Into<FixedPoint<I, P>> + Clone,
        S: Serializer,
        FixedPoint<I, P>: Stringify,
    {
        let mut buf = Default::default();
        fp.clone().into().stringify(&mut buf);
        serializer.serialize_str(buf.as_str())
    }

    /// Deserializes from a string.
    pub fn deserialize<'de, F, I, P, D>(deserializer: D) -> Result<F, D::Error>
    where
        F: From<FixedPoint<I, P>>,
        D: Deserializer<'de>,
        FixedPoint<I, P>: FromStr,
    {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map(F::from).map_err(|_| {
            D::Error::invalid_value(
                de::Unexpected::Str(s),
                &"string containing a fixed-point number",
            )
        })
    }
}

/// (De)serializes `Option<FixedPoint>` as an optional string.
pub mod str_option {
    use super::*;

    /// Serializes to an optional string.
    pub fn serialize<F, I, P, S>(fp: &Option<F>, serializer: S) -> Result<S::Ok, S::Error>
    where
        F: Into<FixedPoint<I, P>> + Clone,
        S: Serializer,
        FixedPoint<I, P>: Stringify,
    {
        if let Some(fp) = fp {
            let mut buf = Default::default();
            fp.clone().into().stringify(&mut buf);
            serializer.serialize_some(buf.as_str())
        } else {
            serializer.serialize_none()
        }
    }

    /// Deserializes from an optional string.
    pub fn deserialize<'de, F, I, P, D>(deserializer: D) -> Result<Option<F>, D::Error>
    where
        F: From<FixedPoint<I, P>>,
        D: Deserializer<'de>,
        FixedPoint<I, P>: FromStr,
    {
        let s = Option::<&str>::deserialize(deserializer)?;
        s.map(|s| {
            s.parse().map(F::from).map_err(|_| {
                D::Error::invalid_value(
                    de::Unexpected::Str(s),
                    &"string containing a fixed-point number",
                )
            })
        })
        .transpose()
    }
}

/// (De)serializes `FixedPoint` as `f64`.
pub mod float {
    use super::*;

    /// Serializes to `f64`.
    #[inline]
    pub fn serialize<F, I, P, S>(fp: &F, serializer: S) -> Result<S::Ok, S::Error>
    where
        F: Into<FixedPoint<I, P>> + Clone,
        I: Serialize,
        FixedPoint<I, P>: Into<f64>,
        S: Serializer,
    {
        serializer.serialize_f64(fp.clone().into().into())
    }

    /// Deserializes from `f64`.
    #[inline]
    pub fn deserialize<'de, F, I, P, D>(deserializer: D) -> Result<F, D::Error>
    where
        F: From<FixedPoint<I, P>>,
        I: Deserialize<'de>,
        FixedPoint<I, P>: TryFrom<f64>,
        D: Deserializer<'de>,
    {
        let f = f64::deserialize(deserializer)?;
        FixedPoint::<I, P>::try_from(f).map(F::from).map_err(|_| {
            D::Error::invalid_value(
                de::Unexpected::Float(f),
                &"float containing a fixed-point number",
            )
        })
    }
}

/// (De)serializes `Option<FixedPoint>` as `Option<f64>`.
pub mod float_option {
    use super::*;

    /// Serializes to `Option<f64>`.
    #[inline]
    pub fn serialize<F, I, P, S>(fp: &Option<F>, serializer: S) -> Result<S::Ok, S::Error>
    where
        F: Into<FixedPoint<I, P>> + Clone,
        I: Serialize,
        FixedPoint<I, P>: Into<f64>,
        S: Serializer,
    {
        if let Some(fp) = fp {
            serializer.serialize_some(&fp.clone().into().into())
        } else {
            serializer.serialize_none()
        }
    }

    /// Deserializes from `Option<f64>`.
    #[inline]
    pub fn deserialize<'de, F, I, P, D>(deserializer: D) -> Result<Option<F>, D::Error>
    where
        F: From<FixedPoint<I, P>>,
        I: Deserialize<'de>,
        FixedPoint<I, P>: TryFrom<f64>,
        D: Deserializer<'de>,
    {
        let f = Option::<f64>::deserialize(deserializer)?;
        f.map(|f| {
            FixedPoint::<I, P>::try_from(f).map(F::from).map_err(|_| {
                D::Error::invalid_value(
                    de::Unexpected::Float(f),
                    &"float containing a fixed-point number",
                )
            })
        })
        .transpose()
    }
}
