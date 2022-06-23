#[test]
fn const_fn() {
    let test_cases = trybuild::TestCases::new();
    test_cases
        .compile_fail("tests/const_fn/01_fixnum_const_bad_str_with_too_long_fractional_part.rs");
}
