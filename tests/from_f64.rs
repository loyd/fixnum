#![cfg(feature = "i128")]

use std::convert::TryInto;

use proptest::prelude::*;

use fixnum::typenum;

type FixedPoint = fixnum::FixedPoint<i128, typenum::U18>;

proptest! {
    #[test]
    fn try_from_f64(x in any::<f64>()) {
        prop_assume!(x.is_finite() && x.abs() <= 1.7014118346046924e20);

        let decimal: Result<rust_decimal::Decimal, _> = x.try_into();
        prop_assume!(decimal.is_ok());
        let decimal = decimal.unwrap();

        let expected = FixedPoint::from_decimal(decimal.mantissa(), -(decimal.scale() as i32));
        prop_assume!(expected.is_ok());
        let expected = expected.unwrap();

        let actual: FixedPoint = x.try_into().expect("cannot convert");
        assert_eq!(actual, expected);
    }
}
