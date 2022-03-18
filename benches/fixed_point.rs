use std::convert::{TryFrom, TryInto};
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

            group.bench_function("cadd", |b| {
                let lhs = black_box($fp::from_bits(123));
                let rhs = black_box($fp::from_bits(54321));
                b.iter(move || lhs.cadd(rhs))
            });

            group.bench_function("rmul", |b| {
                let lhs = black_box($fp::from_bits(123));
                let rhs = black_box($fp::from_bits(54321));
                b.iter(move || lhs.rmul(rhs, RoundMode::Ceil))
            });

            group.bench_function("rdiv", |b| {
                let lhs = black_box($fp::from_bits(987654));
                let rhs = black_box($fp::from_bits(54321));
                b.iter(move || lhs.rdiv(rhs, RoundMode::Ceil))
            });

            group.bench_function("rsqrt (~10^4, precise cases)", |b| {
                let x: $fp = black_box(21234.try_into().unwrap());
                b.iter(move || x.rsqrt(RoundMode::Ceil))
            });

            group.bench_function("rsqrt (deviation)", |b| {
                let x = black_box($fp::MAX);
                b.iter(move || x.rsqrt(RoundMode::Ceil))
            });

            group.bench_function("rsqrt (adaptive)", |b| {
                b.iter_custom(|iters| {
                    let mut num = $fp::ZERO;
                    let step = $fp::MAX
                        .rdiv(*$fp::from_bits(iters as _).as_bits(), RoundMode::Floor)
                        .unwrap();

                    let started_time = Instant::now();
                    for _ in 0..iters {
                        num = num.cadd(step).unwrap();
                        let _ = black_box(num.rsqrt(RoundMode::Ceil));
                    }
                    started_time.elapsed()
                })
            });

            group.bench_function("next_power_of_ten", |b| {
                let mut value = 0;

                b.iter(move || {
                    value += 1;
                    $fp::from_bits(value).next_power_of_ten()
                })
            });

            group.bench_function("FixedPoint::try_from(f64) multiples", |b| {
                let mut value = 1.;

                b.iter(move || {
                    value *= 1.0000001;
                    $fp::try_from(value)
                })
            });

            group.bench_function("FixedPoint::try_from(f64) deviation on MIN_POSITIVE", |b| {
                let value = black_box(f64::MIN_POSITIVE);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) deviation on MAX", |b| {
                let value = black_box(f64::MAX);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) ~10^-12", |b| {
                let value = black_box(3.141592653589793e-12);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) ~0.1", |b| {
                let value = black_box(0.3141592653589793);
                b.iter(move || $fp::try_from(value))
            });

            group.bench_function("FixedPoint::try_from(f64) ~1e6", |b| {
                let value = black_box(3.141592653589793e6);
                b.iter(move || $fp::try_from(value))
            });

            group.finish();
        }
    };
}

#[cfg(feature = "i128")]
define_bench!(F128p18);
#[cfg(feature = "i64")]
define_bench!(F64p9);

#[cfg(all(feature = "i128", feature = "i64"))]
criterion_group!(benches, F128p18, F64p9);
#[cfg(not(feature = "i64"))]
criterion_group!(benches, F128p18);
#[cfg(not(feature = "i128"))]
criterion_group!(benches, F64p9);

criterion_main!(benches);
