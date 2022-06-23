#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::format;

use anyhow::Result;
#[cfg(feature = "i128")]
use proptest::prelude::*;

mod macros;

#[test]
#[allow(overflowing_literals)]
fn from_good_str() -> Result<()> {
    test_fixed_point! {
        case (input: &str, expected: Layout) => {
            let expected = FixedPoint::from_bits(expected);
            let input: FixedPoint = input.parse()?;
            assert_eq!(input, expected);

            #[cfg(feature = "serde")]
            assert_eq!(
                serde_json::from_str::<FixedPoint>(&format!("\"{}\"", input)).unwrap(),
                expected
            );
        },
        fp64 {
            ("1", 1000000000);
            ("1.1", 1100000000);
            ("1.02", 1020000000);
            ("-1.02", -1020000000);
            ("+1.02", 1020000000);
            ("0.1234", 123400000);
            ("-0.1234", -123400000);
            ("123456789.123456789", 123456789123456789);
            ("9223372036.854775807", 9223372036854775807);
            ("-9223372036.854775808", -9223372036854775808);
        },
        fp128 {
            ("1", 1000000000000000000);
            ("1.1", 1100000000000000000);
            ("1.02", 1020000000000000000);
            ("-1.02", -1020000000000000000);
            ("+1.02", 1020000000000000000);
            ("0.1234", 123400000000000000);
            ("-0.1234", -123400000000000000);
            ("123456789.123456789", 123456789123456789000000000);
            ("9223372036.854775807", 9223372036854775807000000000);
            ("-9223372036.854775808", -9223372036854775808000000000);
            ("170141183460469231731.687303715884105727",
             170141183460469231731687303715884105727);
            ("-170141183460469231731.687303715884105728",
             -170141183460469231731687303715884105728);
        },
    };
    Ok(())
}

#[test]
fn from_bad_str() -> Result<()> {
    test_fixed_point! {
        case (bad_str: &str) => {
            let result: Result<FixedPoint, _> = bad_str.parse();
            assert!(result.is_err(), "must not parse '{}'", bad_str);

            #[cfg(feature = "serde")]
            assert!(serde_json::from_str::<FixedPoint>(&format!("\"{}\"", bad_str)).is_err());
        },
        all {
            ("");
            ("7.02e5");
            ("a.12");
            ("12.a");
            ("13.9999999999999999999999999999999999999999999999999999999999999");
            ("100000000000000000000000");
            ("170141183460469231731687303715.884105728");
            ("13.0000000000000000001");
            ("13.1000000000000000001");
            ("9223372036.8547758204856183567");
        },
        fp64 {
            ("13.0000000001");
            ("13.1000000001");
            ("9223372036.854775808");
        },
    };
    Ok(())
}

#[test]
fn display_and_serde() -> Result<()> {
    test_fixed_point! {
        case (x: FixedPoint, expected: &str) => {
            assert_eq!(format!("{}", x), expected);

            #[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

#[cfg(feature = "i128")]
proptest! {
    #[test]
    fn string_roundtrip(x in any::<i128>()) {
        type FixedPoint128 = fixnum::FixedPoint<i128, typenum::U18>;

        let expected = FixedPoint128::from_bits(x);
        let string = expected.to_string();
        let actual: FixedPoint128 = string.parse().unwrap();

        prop_assert_eq!(actual, expected);
    }
}
