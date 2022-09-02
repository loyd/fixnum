use std::time::Instant;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fixnum::{ops::*, FixedPoint};

#[cfg(feature = "i64")]
type F64p9 = FixedPoint<i64, typenum::U9>;
#[cfg(feature = "i128")]
type F128p18 = FixedPoint<i128, typenum::U18>;

macro_rules! define_bench {
    ($fp:tt) => {
        #[allow(non_snake_case)]
        fn $fp(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($fp));

            group.bench_function("cadd (~1e4)", |b| {
                let lhs = black_box($fp::try_from(12345i32).unwrap());
                let rhs = black_box($fp::try_from(54321i32).unwrap());
                b.iter(move || lhs.cadd(rhs))
            });

            let mut rmul = |mode| {
                group.bench_function(format!("rmul (~1e4, {:?})", mode), |b| {
                    let lhs = black_box(
                        $fp::try_from(12345i32)
                            .unwrap()
                            .cadd($fp::from_bits(1))
                            .unwrap(),
                    );
                    let rhs = black_box($fp::from_decimal(5, -1).unwrap());
                    b.iter(move || lhs.rmul(rhs, mode))
                });
            };

            rmul(RoundMode::Floor);
            rmul(RoundMode::Ceil);
            rmul(RoundMode::Nearest);

            let mut rdiv = |mode| {
                group.bench_function(format!("rdiv (~1e5/~1e4, {:?})", mode), |b| {
                    let lhs = black_box($fp::try_from(987656i32).unwrap());
                    let rhs = black_box($fp::try_from(54321i32).unwrap());
                    b.iter(move || lhs.rdiv(rhs, mode))
                });
            };

            rdiv(RoundMode::Floor);
            rdiv(RoundMode::Ceil);
            rdiv(RoundMode::Nearest);

            let mut rsqrt = |mode| {
                group.bench_function(format!("rsqrt (~1e4, {:?})", mode), |b| {
                    let x: $fp = black_box(22347.try_into().unwrap());
                    b.iter(move || x.rsqrt(mode))
                });
            };

            rsqrt(RoundMode::Floor);
            rsqrt(RoundMode::Ceil);
            rsqrt(RoundMode::Nearest);

            let mut rsqrt_max = |mode| {
                group.bench_function(format!("rsqrt (MAX, {:?})", mode), |b| {
                    let x = black_box($fp::MAX);
                    b.iter(move || x.rsqrt(mode))
                });
            };

            rsqrt_max(RoundMode::Floor);
            rsqrt_max(RoundMode::Ceil);
            rsqrt_max(RoundMode::Nearest);

            let mut rsqrt_adaptive = |mode| {
                group.bench_function(format!("rsqrt (adaptive, {:?})", mode), |b| {
                    b.iter_custom(|iters| {
                        let mut num = $fp::ZERO;
                        let step = $fp::MAX
                            .rdiv(*$fp::from_bits(iters as _).as_bits(), RoundMode::Floor)
                            .unwrap();

                        let started_time = Instant::now();
                        for _ in 0..iters {
                            num = num.cadd(step).unwrap();
                            let _ = black_box(num.rsqrt(mode));
                        }
                        started_time.elapsed()
                    })
                });
            };

            rsqrt_adaptive(RoundMode::Floor);
            rsqrt_adaptive(RoundMode::Ceil);
            rsqrt_adaptive(RoundMode::Nearest);

            group.bench_function("next_power_of_ten", |b| {
                let mut value = 0;

                b.iter(move || {
                    value += 1;
                    $fp::from_bits(value).next_power_of_ten()
                })
            });

            group.bench_function("FixedPoint::try_from(f64) (MIN_POSITIVE)", |b| {
                let value = black_box(f64::MIN_POSITIVE);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) (MAX)", |b| {
                let value = black_box(f64::MAX);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) (~1e-12)", |b| {
                let value = black_box(3.141592653589793e-12);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) (~0.1)", |b| {
                let value = black_box(0.3141592653589793);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) (~1e6)", |b| {
                let value = black_box(3.141592653589793e6);
                b.iter(move || $fp::try_from(value))
            });

            group.finish();
        }
    };
}

#[cfg(feature = "i64")]
define_bench!(F64p9);
#[cfg(feature = "i128")]
define_bench!(F128p18);

#[cfg(all(feature = "i64", feature = "i128"))]
criterion_group!(benches, F64p9, F128p18);
#[cfg(not(feature = "i128"))]
criterion_group!(benches, F64p9);
#[cfg(not(feature = "i64"))]
criterion_group!(benches, F128p18);

criterion_main!(benches);
