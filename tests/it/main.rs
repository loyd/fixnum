/// Allows to use generalized `FixedPoint` and `Layout` types in the test cases.
macro_rules! test_fixed_point {
    (
        case ($( $case_pattern:ident: $case_type:ty ),* $( , )?) => $case:block,
        $(
            $section_name:ident {$( ($( $section_args:expr ),* $( , )?) );+ $( ; )?},
        )+
    ) => {{
        macro_rules! impl_test_case {
            () => {
                fn test_case($( $case_pattern: $case_type ),*) -> $crate::TestCaseResult {
                    $case
                    Ok(())
                }
            }
        }

        #[allow(unused)]
        macro_rules! fp {
            ($val:literal) => {{
                // TODO: check that `fixnum!` returns the same value.

                // We don't use `fixnum!` here to check `suite_fails` cases.
                FixedPoint::from_str_exact(stringify!($val))?
            }};
        }

        $(
            test_fixed_point!(@section $section_name {$( ($( $section_args )*) )*});
        )*
    }};
    (case () => $case:block,) => {
        test_fixed_point! {
            case () => $case,
            all {
                ();
            },
        };
    };
    (@section all {$( ($( $args:expr )*) )*}) => {
        #[cfg(feature = "i64")]
        {
            test_fixed_point!(@suite_impl fp64);
            test_fixed_point!(@suite_passes {$( ($( $args )*) )*});
        }
        #[cfg(feature = "i128")]
        {
            test_fixed_point!(@suite_impl fp128);
            test_fixed_point!(@suite_passes {$( ($( $args )*) )*});
        }
    };
    (@section fp64 {$( ($( $args:expr )*) )*}) => {
        #[cfg(feature = "i64")]
        {
            test_fixed_point!(@suite_impl fp64);
            test_fixed_point!(@suite_passes {$( ($( $args )*) )*});
        }
        #[cfg(feature = "i128")]
        {
            test_fixed_point!(@suite_impl fp128);
            test_fixed_point!(@suite_fails {$( ($( $args )*) )*});
        }
    };
    (@section fp128 {$( ($( $args:expr )*) )*}) => {
        #[cfg(feature = "i128")]
        {
            test_fixed_point!(@suite_impl fp128);
            test_fixed_point!(@suite_passes {$( ($( $args )*) )*});
        }
        #[cfg(feature = "i64")]
        {
            test_fixed_point!(@suite_impl fp64);
            test_fixed_point!(@suite_fails {$( ($( $args )*) )*});
        }
    };
    (@suite_impl fp64) => {
        type Layout = i64;
        #[allow(unused)]
        type FixedPoint = fixnum::FixedPoint<Layout, typenum::U9>;
        impl_test_case!();
    };
    (@suite_impl fp128) => {
        type Layout = i128;
        #[allow(unused)]
        type FixedPoint = fixnum::FixedPoint<Layout, typenum::U18>;
        impl_test_case!();
    };
    (@suite_passes {$( ($( $args:expr )*) )*}) => {
        $(
            $crate::r#impl::catch_and_augment(stringify!($( $args ),*), || {
                test_case($( $args ),*)
            })?;
        )*
    };
    (@suite_fails {$( ($( $args:expr )*) )*}) => {
        $(
            $crate::r#impl::catch_and_augment(stringify!($( $args ),*), || {
                $crate::r#impl::assert_fails(|| test_case($( $args ),*));
                Ok(())
            })?;
        )*
    };
}

use std::fmt::Display;

// Use a special error based on `Display` in order to support `nostd`.
type TestCaseResult = Result<(), TestCaseError>;
struct TestCaseError(Box<dyn Display>);

impl<E: Display + 'static> From<E> for TestCaseError {
    fn from(error: E) -> Self {
        Self(Box::new(error))
    }
}

impl From<TestCaseError> for anyhow::Error {
    fn from(error: TestCaseError) -> Self {
        // Avoid calling `anyhow!(error)` here to support `TestCaseError(TestCaseError)`.
        anyhow::anyhow!(error.0.to_string())
    }
}

#[cfg(not(feature = "std"))]
mod r#impl {
    use anyhow::Result;

    use super::TestCaseResult;

    pub(crate) fn assert_fails(_case: impl FnOnce() -> TestCaseResult) {}

    pub(crate) fn catch_and_augment(
        _name: &'static str,
        case: impl FnOnce() -> TestCaseResult,
    ) -> Result<()> {
        case().map_err(Into::into)
    }
}

#[cfg(feature = "std")]
mod r#impl {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use anyhow::{anyhow, Context, Result};
    use colored::Colorize;

    use super::TestCaseResult;

    pub(crate) fn assert_fails(case: impl FnOnce() -> TestCaseResult) {
        assert!(
            !matches!(catch_unwind(AssertUnwindSafe(case)), Ok(Ok(()))),
            "must fail, but successed"
        );
    }

    pub(crate) fn catch_and_augment(
        name: &'static str,
        case: impl FnOnce() -> TestCaseResult,
    ) -> Result<()> {
        // TODO: the implementation isn't ideal and prints the panic twice.
        // A better solution requires a custom panic hook and manual backtrace handling.
        let result = match catch_unwind(AssertUnwindSafe(case)) {
            Ok(res) => res.map_err(Into::into),
            Err(panic) => Err(anyhow!(stringify_panic(panic))),
        };

        result.context(format!("\n\n    case {} failed", name.blue()))
    }

    fn stringify_panic(payload: Box<dyn std::any::Any>) -> String {
        if let Some(message) = payload.downcast_ref::<&str>() {
            format!("panic: {}", message)
        } else if let Some(message) = payload.downcast_ref::<String>() {
            format!("panic: {}", message)
        } else {
            "panic: <unsupported payload>".into()
        }
    }
}

// Tests
mod const_ctor;
mod convert;
mod convert_f64;
mod convert_str;
mod ops;
mod rkyv;
mod serde;
