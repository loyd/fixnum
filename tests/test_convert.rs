use anyhow::Result;
#[cfg(feature = "i128")]
use proptest::prelude::*;

use fixnum::ops::Bounded;

mod macros;

#[test]
fn from_decimal() -> Result<()> {
    test_fixed_point! {
        case (input: (Layout, i32), expected: FixedPoint) => {
            assert_eq!(FixedPoint::from_decimal(input.0, input.1)?, expected);
            assert_eq!(FixedPoint::from_decimal(-input.0, input.1)?, expected.cneg().unwrap());
        },
        all {
            ((5_000_000_000, -9), fp!(5));
            ((1, 0), fp!(1));
            ((1, 1), fp!(10));
        },
        fp128 {
            ((5_000_000_000_000_000_000, -18), fp!(5));
        },
    };
    Ok(())
}

#[test]
fn to_decimal() -> Result<()> {
    test_fixed_point! {
        case (fp: FixedPoint, max_exponent: i32, expected: (Layout, i32)) => {
            let (mantissa, exponent) = fp.to_decimal(max_exponent);
            assert!(exponent <= max_exponent);
            assert_eq!((mantissa, exponent), expected);

            let (mantissa, exponent) = fp.cneg().unwrap().to_decimal(max_exponent);
            assert!(exponent <= max_exponent);
            assert_eq!((mantissa, exponent), (-expected.0, expected.1));
        },
        all {
            (fp!(0), 0, (0, 0));
            (fp!(0), -5, (0, -5));
            (fp!(0), 5, (0, 0));
            (fp!(50), 0, (50, 0));
            (fp!(50), 1, (5, 1));
            (fp!(50), 5, (5, 1));
            (fp!(50), i32::MAX, (5, 1));
            (fp!(50), -2, (5000, -2));
            (fp!(5.5), 0, (55, -1));
            (fp!(5.5), 1, (55, -1));
            (fp!(5.5), 5, (55, -1));
            (fp!(5.5), -1, (55, -1));
            (fp!(5.5), -2, (550, -2));
            (FixedPoint::MAX, 0, (FixedPoint::MAX.into_bits(), -FixedPoint::PRECISION))
        },
    };
    Ok(())
}

#[cfg(feature = "i128")]
proptest! {
    #[test]
    fn decimal_roundtrip(bits in any::<i128>(), zeros in 0u32..=38) {
        type FixedPoint128 = fixnum::FixedPoint<i128, typenum::U18>;

        let zeros = 10i128.pow(zeros);
        let expected = FixedPoint128::from_bits(bits / zeros * zeros);

        for max_exponent in -18..=5 {
            let (mantissa, exponent) = expected.to_decimal(max_exponent);
            let actual = FixedPoint128::from_decimal(mantissa, exponent).unwrap();
            prop_assert!(-FixedPoint128::PRECISION <= exponent && exponent <= max_exponent);
            prop_assert_eq!(actual, expected);

            if mantissa != 0 && exponent != max_exponent {
                prop_assert_ne!(mantissa % 10, 0);
            }
        }
    }
}
