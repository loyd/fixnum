#![cfg(feature = "rkyv")]

use fixnum::*;
use anyhow::Result;
use rkyv::rancor::Error;

#[test]
fn rkyv_basic() -> Result<()> {
    test_fixed_point! {
        case (a: FixedPoint, b: FixedPoint) => {
            let a_ = rkyv::to_bytes::<Error>(&a)?;
            let b_ = rkyv::to_bytes::<Error>(&b)?;
            assert_eq!(
                rkyv::deserialize::<FixedPoint, Error>(
                    rkyv::access::<<FixedPoint as rkyv::Archive>::Archived, Error>(&a_)?)?, a);
            assert_eq!(
                rkyv::deserialize::<FixedPoint, Error>(
                    rkyv::access::<<FixedPoint as rkyv::Archive>::Archived, Error>(&b_)?)?, b);


            let ab = vec![a,b];
            let ab_ = rkyv::to_bytes::<Error>(&ab)?;
            let ab_access = rkyv::access::<<Vec<FixedPoint> as rkyv::Archive>::Archived, Error>(&ab_)?;
            assert_eq!(rkyv::deserialize::<FixedPoint, Error>(&ab_access[0])?, a);
            assert_eq!(rkyv::deserialize::<FixedPoint, Error>(&ab_access[1])?, b);
        },
        all {
            (fp!(64.0), fp!(13.13));
        },
    };
    Ok(())
}

