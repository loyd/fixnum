#[cfg(feature = "i64")]
#[test]
fn valid() {
    use fixnum::{fixnum_const, FixedPoint};
    type F64p9 = FixedPoint<i64, typenum::U9>;

    const SAMPLE0: F64p9 = fixnum_const!(42.42, 9);
    assert_eq!(SAMPLE0, F64p9::from_decimal(4242, -2).unwrap());

    const SAMPLE1: F64p9 = fixnum_const!(42, 9);
    assert_eq!(SAMPLE1, F64p9::from_decimal(42, 0).unwrap());

    const SAMPLE2: F64p9 = fixnum_const!(42., 9);
    assert_eq!(SAMPLE2, F64p9::from_decimal(42, 0).unwrap());
}

#[test]
fn too_long_fractional() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/it/const_ctor/too_long_fractional.rs");
}
