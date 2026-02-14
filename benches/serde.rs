use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};

use fixnum::{fixnum, ops::*, FixedPoint};

#[cfg(feature = "i64")]
type F64p9 = FixedPoint<i64, typenum::U9>;
#[cfg(feature = "i128")]
type F128p18 = FixedPoint<i128, typenum::U18>;

macro_rules! define_bench {
    ($fp:tt, $coef:literal) => {
        #[allow(non_snake_case)]
        fn $fp(c: &mut Criterion) {
            #[derive(Serialize, Deserialize)]
            #[serde(transparent)]
            struct AsString {
                #[serde(with = "fixnum::serde::str")]
                v: $fp,
            }

            #[derive(Serialize, Deserialize)]
            #[serde(transparent)]
            struct AsFloat {
                #[serde(with = "fixnum::serde::float")]
                v: $fp,
            }

            let mut group = c.benchmark_group(stringify!($fp));

            group.bench_function("serialize 123.456 to string", |b| {
                let fp = black_box(AsString {
                    v: fixnum!(123.456, $coef),
                });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("serialize MAX to string", |b| {
                let fp = black_box(AsString { v: $fp::MAX });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("deserialize 123.456 from string", |b| {
                let s = black_box(format!("\"123.456\""));
                b.iter(move || {
                    let fp: AsString = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("deserialize MAX from string", |b| {
                let s = black_box(format!("\"{}\"", $fp::MAX));
                b.iter(move || {
                    let fp: AsString = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            // Scientific notation (positive exponent)
            group.bench_function("deserialize 1.23e4 from string", |b| {
                let s = black_box(format!("\"1.23e4\""));
                b.iter(move || {
                    let fp: AsString = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            // Scientific notation (MAX with e0)
            group.bench_function("deserialize MAXe0 from string", |b| {
                let s = black_box(format!("\"{}e0\"", $fp::MAX));
                b.iter(move || {
                    let fp: AsString = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("serialize 123.456 to f64", |b| {
                let fp = black_box(AsFloat {
                    v: fixnum!(123.456, $coef),
                });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("serialize MAX to f64", |b| {
                let fp = black_box(AsFloat { v: $fp::MAX });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("deserialize 123.456 from f64", |b| {
                let s = black_box(format!("123.456"));
                b.iter(move || {
                    let fp: AsFloat = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("deserialize MAX from f64", |b| {
                let s = black_box(format!("{}", f64::from($fp::MAX) * 0.9));
                b.iter(move || {
                    let fp: AsFloat = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });
        }
    };
}

#[cfg(feature = "i64")]
define_bench!(F64p9, 9);
#[cfg(feature = "i128")]
define_bench!(F128p18, 18);

#[cfg(all(feature = "i64", feature = "i128"))]
criterion_group!(benches, F64p9, F128p18);
#[cfg(not(feature = "i128"))]
criterion_group!(benches, F64p9);
#[cfg(not(feature = "i64"))]
criterion_group!(benches, F128p18);

criterion_main!(benches);
