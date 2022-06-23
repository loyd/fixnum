use anyhow::Result;

mod macros;

#[test]
fn from_decimal() -> Result<()> {
    test_fixed_point! {
        case (numerator: Layout, denominator: i32, expected: FixedPoint) => {
            assert_eq!(FixedPoint::from_decimal(numerator, denominator)?, expected);
        },
        all {
            (5_000_000_000, -9, fp!(5));
            (1, 0, fp!(1));
            (1, 1, fp!(10));
        },
        fp128 {
            (5_000_000_000_000_000_000, -18, fp!(5));
        },
    };
    Ok(())
}

#[test]
fn rounding_to_i64() -> Result<()> {
    test_fixed_point! {
        case (x: FixedPoint, expected: i64) => {
            assert_eq!(x.rounding_to_i64(), expected);
        },
        all {
            (fp!(0), 0);
            (fp!(42), 42);
            (fp!(1.4), 1);
            (fp!(1.6), 2);
            (fp!(-1.4), -1);
            (fp!(-1.6), -2);
            (fp!(0.4999), 0);
            (fp!(0.5), 1);
            (fp!(0.5001), 1);
        },
    };
    Ok(())
}
