#![no_main]
use std::convert::TryInto;

use libfuzzer_sys::fuzz_target;

use fixnum::typenum;

type FixedPoint = fixnum::FixedPoint<i128, typenum::U18>;

// Checks FixedPoint::try_from(f64) against corresponding rust_decimal implementation
fuzz_target!(|x: f64| {
    let actual: FixedPoint = match x.try_into() {
        Ok(val) => val,
        Err(err) => {
            if !x.is_finite() || x.abs() > 1.7014118346046924e20 {
                return;
            } else {
                panic!("cant convert f64 {} {}", x, err);
            }
        }
    };
    let decimal: rust_decimal::Decimal = match x.try_into() {
        Ok(val) => val,
        Err(_) => return,
    };
    let expected = match FixedPoint::from_decimal(decimal.mantissa(), -(decimal.scale() as i32)) {
        Ok(val) => val,
        Err(_) => return,
    };
    assert_eq!(actual, expected);
});
