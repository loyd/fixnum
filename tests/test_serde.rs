#![cfg(feature = "serde")]
use anyhow::Result;

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
        case (input: f64, expected: FixedPoint) => {
            #[derive(::serde::Serialize, ::serde::Deserialize)]
            struct Struct {
                #[serde(with = "fixnum::serde::as_f64")]
                number: FixedPoint,
            }

            let actual = serde_json::from_str::<Struct>(&format!(r#"{{"number":{}}}"#, input)).unwrap().number;
            assert_eq!(expected, actual);
        },
        all {
            (1., fp!(1.0));
            (1.1, fp!(1.1));
            (1.02, fp!(1.02));
            (-1.02, fp!(-1.02));
            (0.1234, fp!(0.1234));
            (-0.1234, fp!(-0.1234));
        },
    };
    Ok(())
}
