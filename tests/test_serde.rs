#![cfg(feature = "serde")]

use anyhow::Result;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

mod macros;

#[test]
fn display_and_serde() -> Result<()> {
    test_fixed_point! {
        case (x: FixedPoint, expected: &str) => {
            assert_eq!(x.to_string(), expected);
            assert_eq!(serde_json::to_string(&x).unwrap(), format!("\"{}\"", expected));
        },
        all {
            (fp!(0), "0.0");
            (fp!(42), "42.0");
            (fp!(10.042), "10.042");
            (fp!(-10.042), "-10.042");
            (fp!(0.000000001), "0.000000001");
            (fp!(-0.000000001), "-0.000000001");
            (fp!(9223372036.854775807), "9223372036.854775807");
            (fp!(-9223372036.854775808), "-9223372036.854775808");
        },
        fp128 {
            (fp!(0.000000000000000001), "0.000000000000000001");
            (fp!(-0.000000000000000001), "-0.000000000000000001");
            (fp!(170141183460469231731.687303715884105727), "170141183460469231731.687303715884105727");
            (fp!(-170141183460469231731.687303715884105728), "-170141183460469231731.687303715884105728");
        },
    };
    Ok(())
}

#[test]
fn deserialize() -> Result<()> {
    test_fixed_point! {
        case (json: &str, expected: FixedPoint) => {
            let actual: FixedPoint = serde_json::from_str(&json).unwrap();
            assert_eq!(actual, expected);

            let actual: FixedPoint = serde_json::from_str(&format!("\"{}\"", json)).unwrap();
            assert_eq!(actual, expected);
        },
        all {
            ("42", fp!(42));
            ("-42", fp!(-42));
            ("42.0", fp!(42));
            ("-42.0", fp!(-42));
            ("42.1", fp!(42.1));
            ("-42.1", fp!(-42.1));
        },
        // TODO: check `i128`/`u128` (using bincode?)
    };
    Ok(())
}

#[test]
fn serde_with() -> Result<()> {
    test_fixed_point! {
        case (value: FixedPoint) => {
            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
            struct Sample {
                #[serde(with = "fixnum::serde::repr")]
                repr: FixedPoint,
                #[serde(with = "fixnum::serde::str")]
                str: FixedPoint,
                #[serde(with = "fixnum::serde::float")]
                float: FixedPoint,
            }

            #[derive(Debug, Clone, PartialEq, Eq, Into, From)]
            struct Amount(FixedPoint);

            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
            struct WrappedSample {
                #[serde(with = "fixnum::serde::repr")]
                repr: Amount,
                #[serde(with = "fixnum::serde::str")]
                str: Amount,
                #[serde(with = "fixnum::serde::float")]
                float: Amount,
            }

            #[derive(Debug, PartialEq, Deserialize)]
            struct Raw {
                repr: Layout,
                str: String,
                float: f64,
            }

            let sample = Sample {
                repr: value,
                str: value,
                float: value,
            };
            let wrapped_sample = WrappedSample {
                repr: value.into(),
                str: value.into(),
                float: value.into(),
            };

            // Check raw representation.
            let json = serde_json::to_string(&sample).unwrap();
            assert_eq!(serde_json::to_string(&wrapped_sample).unwrap(), json);

            let raw: Raw = serde_json::from_str(&json).unwrap();
            assert_eq!(raw, Raw {
                repr: value.into_bits(),
                str: value.to_string(),
                float: value.into(),
            });

            // Check round-trip.
            let actual: Sample = serde_json::from_str(&json).unwrap();
            assert_eq!(actual, sample);
            let actual: WrappedSample = serde_json::from_str(&json).unwrap();
            assert_eq!(actual, wrapped_sample);
        },
        all {
            (fp!(0));
            (fp!(1));
            (fp!(1.1));
            (fp!(1.02));
            (fp!(-1.02));
            (fp!(0.1234));
            (fp!(-0.1234));
        },
    };
    Ok(())
}
