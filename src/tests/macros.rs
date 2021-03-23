pub fn assert_fails<F: FnOnce() -> anyhow::Result<()>>(#[allow(unused)] case: F) {
    #[cfg(feature = "std")]
    {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        assert!(!matches!(catch_unwind(AssertUnwindSafe(case)), Ok(Ok(()))));
    }
}

/// Testing helper macro. Allows to use generalized `FixedPoint` and `Layout` types in the test cases.
#[macro_export]
macro_rules! test_fixed_point {
    (
        case ($( $case_pattern:pat | $case_type:ty ),* $( , )?) => $case:block,
        $(
            $section_name:ident {$( ($( $section_args:expr ),* $( , )?) );+ $( ; )?},
        )+
    ) => {{
        macro_rules! impl_test_case {
            () => {
                fn test_case($( $case_pattern : $case_type ),*) -> Result<()> {
                    $case
                    Ok(())
                }
            }
        }

        #[allow(unused)]
        macro_rules! fp {
            ($val:literal) => {{
                let value: FixedPoint = stringify!($val).parse()?;
                value
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
        {
            test_fixed_point!(@suite_impl fp64);
            test_fixed_point!(@suite_fails {$( ($( $args )*) )*});
        }
    };
    (@suite_impl fp64) => {
        type Layout = i64;
        #[allow(unused)]
        type FixedPoint = crate::FixedPoint<Layout, typenum::U9>;
        impl_test_case!();
    };
    (@suite_impl fp128) => {
        type Layout = i128;
        #[allow(unused)]
        type FixedPoint = crate::FixedPoint<Layout, typenum::U18>;
        impl_test_case!();
    };
    (@suite_passes {$( ($( $args:expr )*) )*}) => {
        $(
            test_case($( $args ),*)?;
        )*
    };
    (@suite_fails {$( ($( $args:expr )*) )*}) => {
        $(
            $crate::tests::macros::assert_fails(|| -> anyhow::Result<()> {
                test_case($( $args ),*)
            });
        )*
    };
}
