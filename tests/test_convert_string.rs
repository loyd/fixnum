#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::format;

use anyhow::Result;
#[cfg(feature = "i128")]
use proptest::prelude::*;

use self::macros::TestCaseResult;

mod macros;

#[test]
#[allow(overflowing_literals)]
fn from_good_str_exact() -> Result<()> {
    test_fixed_point! {
        case (input: &str, expected: Layout) => {
            let expected = FixedPoint::from_bits(expected);
            let exact = FixedPoint::from_str_exact(input)?;
            assert_eq!(exact, expected);

            // A rounding version must return same result.
            let inexact: FixedPoint = input.parse()?;
            assert_eq!(inexact, exact);

            #[cfg(feature = "serde")]
            assert_eq!(serde_json::from_str::<FixedPoint>(&format!("\"{}\"", input))?, expected);
            #[cfg(feature = "serde")]
            assert_eq!(serde_json::from_str::<FixedPoint>(&format!("\"{}\"", exact))?, expected);
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
#[allow(overflowing_literals)]
fn from_good_str_inexact() -> Result<()> {
    test_fixed_point! {
        case (input: &str, expected: Layout) => {
            fn check(input: &str, expected: Layout) -> TestCaseResult {
                let expected = FixedPoint::from_bits(expected);
                let inexact: FixedPoint = input.parse()?;
                assert_eq!(inexact, expected);

                // An exact version must fail.
                let exact = FixedPoint::from_str_exact(input);
                assert!(exact.is_err(), "exact must not parse '{}'", input);

                #[cfg(feature = "serde")]
                assert_eq!(serde_json::from_str::<FixedPoint>(&format!("\"{}\"", input))?, expected);
                #[cfg(feature = "serde")]
                assert_eq!(serde_json::from_str::<FixedPoint>(&format!("\"{}\"", inexact))?, expected);
                Ok(())
            }

            check(input, expected)?;
            check(&format!("-{}", input), -expected)?;
        },
        fp64 {
            ("13.0000000000000000001", 13000000000);
            ("13.0000000000000000005", 13000000000);
            ("13.0000000001", 13000000000);
            ("13.0000000005", 13000000001);
            ("13.0000000009", 13000000001);
            ("13.1000000001", 13100000000);
            ("13.1000000005", 13100000001);
            ("13.1000000009", 13100000001);
            ("13.9999999999999999999999999999999999999999999999999999999999999", 14000000000);
            ("9223372036.8547758071", 9223372036854775807);
            // Negative cases are also checked.
        },
        fp128 {
            ("13.0000000000000000001", 13000000000000000000);
            ("13.0000000000000000005", 13000000000000000001);
            ("13.0000000000000000009", 13000000000000000001);
            ("13.9999999999999999999999999999999999999999999999999999999999999",
             14000000000000000000);
            ("170141183460469231731.6873037158841057271",
             170141183460469231731687303715884105727)
            // Negative cases are also checked.
        },
    };
    Ok(())
}

#[test]
fn from_bad_str() -> Result<()> {
    test_fixed_point! {
        case (bad_str: &str) => {
            let exact = FixedPoint::from_str_exact(bad_str);
            let inexact: Result<FixedPoint, _> = bad_str.parse();
            assert!(exact.is_err(), "exact must not parse '{}'", bad_str);
            assert!(inexact.is_err(), "inexact must not parse '{}'", bad_str);

            #[cfg(feature = "serde")]
            assert!(serde_json::from_str::<FixedPoint>(&format!("\"{}\"", bad_str)).is_err());
        },
        all {
            ("");
            ("7.02e5");
            ("a.12");
            ("12.a");
            ("100000000000000000000000");
            ("170141183460469231731687303715.884105728");
            ("170141183460469231731.687303715884105728");
            ("170141183460469231731.6873037158841057275");
            ("-170141183460469231731.687303715884105729");
            ("-170141183460469231731.6873037158841057285");
        },
        fp64 {
            ("9223372036.854775808");
            ("9223372036.8547758075");
            ("-9223372036.854775809");
            ("-9223372036.8547758085");
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

        let exact = FixedPoint128::from_str_exact(&string).unwrap();
        let inexact: FixedPoint128 = string.parse().unwrap();

        prop_assert_eq!(inexact, expected);
        prop_assert_eq!(exact, expected);
    }
}
