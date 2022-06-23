#![cfg(feature = "i128")]

use std::convert::TryInto;

use proptest::prelude::*;

use fixnum::typenum;

type FixedPoint = fixnum::FixedPoint<i128, typenum::U18>;

const MAX_F64: f64 = 1.7014118346046924e20;

proptest! {
    #[test]
    fn try_from_f64(x in -MAX_F64..MAX_F64) {
        let decimal: Result<rust_decimal::Decimal, _> = x.try_into();
        prop_assume!(decimal.is_ok());
        let decimal = decimal.unwrap();

        let expected = FixedPoint::from_decimal(decimal.mantissa(), -(decimal.scale() as i32));
        prop_assume!(expected.is_ok());
        let expected = expected.unwrap();

        let actual: FixedPoint = x.try_into().expect("cannot convert");
        prop_assert_eq!(actual, expected);
    }
}

proptest! {
    #[test]
    fn string_roundtrip(x in any::<i128>()) {
        let expected = FixedPoint::from_bits(x);
        let string = expected.to_string();
        let actual: FixedPoint = string.parse().unwrap();

        prop_assert_eq!(actual, expected);
    }
}
