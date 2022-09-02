use anyhow::Result;
#[cfg(feature = "i128")]
use proptest::prelude::*;

use fixnum::*;

mod macros;

#[test]
#[allow(clippy::float_cmp)]
fn to_f64() -> Result<()> {
    test_fixed_point! {
        case (x: FixedPoint, expected: f64) => {
            assert_eq!(f64::from(x), expected);
        },
        all {
            (fp!(0), 0.0);
            (fp!(0.1), 0.1);
            (fp!(1), 1.0);
            (fp!(1.5), 1.5);
            (fp!(-5), -5.);
            (fp!(-14.14), -14.14);
            (fp!(42.123456789), 42.123456789);
            (fp!(-42.123456789), -42.123456789);
            (fp!(8003332421.536753168), 8003332421.536753);
            (fp!(-8003332421.536753168), -8003332421.536753);
            (fp!(9223372036.854775807), 9223372036.854776);
            (fp!(-9223372036.854775807), -9223372036.854776);
            (fp!(922337203.685477581), 922337203.6854776);
            (fp!(-922337203.685477581), -922337203.6854776);
            (fp!(92233720.36854775), 92233720.36854775);
            (fp!(-92233720.36854775), -92233720.36854775);
        },
        fp128 {
            (fp!(0.0000000000025), 25e-13);
            (fp!(1000000.0000000000025), 1e6);
            (fp!(-1000000.0000000000025), -1e6);
            (fp!(0.000000000000000025), 25e-18);
            (fp!(-0.000000000000000025), -25e-18);
            (fp!(2.1234567890123457), 2.1234567890123457);
            (fp!(-2.1234567890123457), -2.1234567890123457);
        },
    };
    Ok(())
}

#[test]
fn from_f64() -> Result<()> {
    test_fixed_point! {
        case (expected: FixedPoint, x: f64) => {
            assert_eq!(FixedPoint::try_from(x)?, expected);
            assert_eq!(FixedPoint::try_from(-x)?, expected.cneg()?);
        },
        all {
            (fp!(0), 0.0);
            (fp!(0.5), 0.5);
            (fp!(1), 1.0);
            (fp!(1.5), 1.5);
            (fp!(42.123456789), 42.123456789);

            (fp!(803332.421536753), 803332.421536753);
            (fp!(8033324.21536753), 8033324.21536753);
            (fp!(80333242.1536753), 80333242.1536753);
            (fp!(803332421.536753), 803332421.536753);
            (fp!(8033324215.36753), 8033324215.36753);

            (fp!(9223372036.85477), 9223372036.85477);
        },
        fp128 {
            (fp!(0.803332421536753), 0.803332421536753);
            (fp!(8.03332421536753), 8.03332421536753);
            (fp!(8.03332421536753), 8.03332421536753);
            (fp!(80.3332421536753), 80.3332421536753);
            (fp!(803.332421536753), 803.332421536753);
            (fp!(8033.32421536753), 8033.32421536753);
            (fp!(80333.2421536753), 80333.2421536753);
            // <see part of cases in `all` sections>
            (fp!(80333242153.6753), 80333242153.6753);
            (fp!(803332421536.753), 803332421536.753);
            (fp!(8033324215367.53), 8033324215367.53);
            (fp!(80333242153675.3), 80333242153675.3);
            (fp!(803332421536753), 803332421536753.);
            (fp!(8033324215367530), 8033324215367530.);
            (fp!(8033324215367533), 8033324215367533.);

            (fp!(170141183460469211136), 1.701411834604692e20);
        },
    };
    Ok(())
}

#[test]
fn from_f64_exact() -> Result<()> {
    test_fixed_point! {
        case (x: f64, expected: FixedPoint) => {
            assert_eq!(FixedPoint::try_from(x)?, expected);
            assert_eq!(FixedPoint::try_from(-x)?, expected.cneg()?);
        },
        all {
            (f64::MIN_POSITIVE, fp!(0));
            (1e-19, fp!(0));
            (0.000006927_f64, fp!(0.000006927));
            (0.00006927_f64, fp!(0.00006927));
            (0.1_f64, fp!(0.1));
            (0.12345_f64, fp!(0.12345));
            (0.6927_f64, fp!(0.6927));
            (5.1_f64, fp!(5.1));
            (1643804666.060961, fp!(1643804666.060961));
        },
        fp64 {
            (f64::EPSILON, fp!(0));
            (0.123_456_789_412_345_61_f64, fp!(0.123456789));
            (0.123_456_789_512_345_61_f64, fp!(0.12345679));
        },
        fp128 {
            (f64::EPSILON, fp!(0.000000000000000222));
            (0.123_456_789_012_345_61_f64, fp!(0.1234567890123456));
            (0.123_456_789_012_345_68_f64, fp!(0.1234567890123457));
        },
    };
    Ok(())
}

#[test]
fn from_f64_limits() -> Result<()> {
    test_fixed_point! {
        case (x: f64, expected: &str) => {
            let actual = FixedPoint::try_from(x).map_err(|err| err.to_string());
            assert_eq!(actual, Err(expected.to_string()));
        },
        all {
            (f64::NAN, "not finite");
            (f64::INFINITY, "not finite");
            (f64::NEG_INFINITY, "not finite");
            (f64::MAX, "too big number");
            (f64::MIN, "too big number");
        },
    };
    Ok(())
}

#[cfg(feature = "i128")]
const MAX_F64: f64 = 1.7014118346046924e20;

#[cfg(feature = "i128")]
proptest! {
    /// Checks that `f64 -> fixnum` equals to `f64 -> rust_decimal -> fixnum`.
    #[test]
    fn compare_with_rust_decimal(x in -MAX_F64..MAX_F64) {
        use core::convert::TryInto;

        type FixedPoint128 = fixnum::FixedPoint<i128, typenum::U18>;

        let decimal: Result<rust_decimal::Decimal, _> = x.try_into();
        prop_assume!(decimal.is_ok());
        let decimal = decimal.unwrap();

        let expected = FixedPoint128::from_decimal(decimal.mantissa(), -(decimal.scale() as i32));
        prop_assume!(expected.is_ok());
        let expected = expected.unwrap();

        let actual: FixedPoint128 = x.try_into().expect("cannot convert");
        prop_assert_eq!(actual, expected);
    }
}
