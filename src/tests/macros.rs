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
        test_fixed_point!(@section fp64 {$( ($( $args )*) )*});
        test_fixed_point!(@section fp128 {$( ($( $args )*) )*});
    };
    (@section fp64 {$( ($( $args:expr )*) )*}) => {{
        type Layout = i64;
        type FixedPoint = crate::FixedPoint<Layout, typenum::U9>;

        #[allow(unused)]
        macro_rules! fp {
            ($val:literal) => {
                $crate::fixnum!($val, 9)
            };
        }

        test_fixed_point!(@suite {$( ($( $args )*) )*});
    }};
    (@section fp128 {$( ($( $args:expr )*) )*}) => {
        #[cfg(feature = "i128")]
        {
            type Layout = i128;
            type FixedPoint = crate::FixedPoint<Layout, typenum::U18>;

            #[allow(unused)]
            macro_rules! fp {
                ($val:literal) => {
                    $crate::fixnum!($val, 18)
                };
            }

            test_fixed_point!(@suite {$( ($( $args )*) )*});
        }
    };
    (@suite {$( ($( $args:expr )*) )*}) => {{
        impl_test_case!();
        $(
            test_case($( $args ),*)?;
        )*
    }};
}
