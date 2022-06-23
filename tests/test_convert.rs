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
