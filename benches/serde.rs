use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fixnum::{ops::*, FixedPoint};

type F32p5 = FixedPoint<i32, typenum::U5>;
type F64p9 = FixedPoint<i64, typenum::U9>;
#[cfg(feature = "i128")]
type F128p18 = FixedPoint<i128, typenum::U18>;

macro_rules! define_bench {
    ($fp:tt) => {
        #[allow(non_snake_case)]
        fn $fp(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($fp));

            group.bench_function("serialize (small)", |b| {
                let fp = black_box($fp::from_bits(123456));
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("serialize (max)", |b| {
                let fp = black_box($fp::MAX);
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("deserialize (small)", |b| {
                let s = black_box(format!("\"123456\""));
                b.iter(move || {
                    let fp: $fp = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("deserialize (max)", |b| {
                let s = black_box(format!("\"{}\"", $fp::MAX));
                b.iter(move || {
                    let fp: $fp = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });
        }
    };
}

#[cfg(feature = "i128")]
define_bench!(F128p18);
define_bench!(F64p9);
define_bench!(F32p5);

#[cfg(feature = "i128")]
criterion_group!(benches, F128p18, F64p9, F32p5);
#[cfg(not(feature = "i128"))]
criterion_group!(benches, F64p9, F32p5);

criterion_main!(benches);
