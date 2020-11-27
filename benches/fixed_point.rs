use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fixnum::{ops::*, FixedPoint};

type F32p5 = FixedPoint<i32, typenum::U5>;
type F64p9 = FixedPoint<i64, typenum::U9>;
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

            //group.bench_function("next_power_of_ten", |b| {
            //let mut value = 0;

            //b.iter(move || {
            //value += 1;
            //$fp::from_bits(value).next_power_of_ten()
            //})
            //});

            group.finish();
        }
    };
}

define_bench!(F128p18);
define_bench!(F64p9);
define_bench!(F32p5);

criterion_group!(benches, F128p18, F64p9, F32p5);
criterion_main!(benches);
