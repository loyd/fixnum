use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};

use fixnum::{ops::*, FixedPoint};

type F64p9 = FixedPoint<i64, typenum::U9>;
#[cfg(feature = "i128")]
type F128p18 = FixedPoint<i128, typenum::U18>;

macro_rules! define_bench {
    ($fp:tt) => {
        #[allow(non_snake_case)]
        fn $fp(c: &mut Criterion) {
            #[derive(Serialize, Deserialize)]
            #[serde(transparent)]
            struct AsString {
                #[serde(with = "fixnum::serde::as_string")]
                v: $fp,
            }

            #[derive(Serialize, Deserialize)]
            #[serde(transparent)]
            struct AsF64 {
                #[serde(with = "fixnum::serde::as_f64")]
                v: $fp,
            }

            let mut group = c.benchmark_group(stringify!($fp));

            group.bench_function("serialize: string, short", |b| {
                let fp = black_box(AsString {
                    v: $fp::from_bits(123456),
                });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("serialize: string, long", |b| {
                let fp = black_box(AsString { v: $fp::MAX });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("deserialize: string, short", |b| {
                let s = black_box(format!("\"123456\""));
                b.iter(move || {
                    let fp: AsString = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("deserialize string, long", |b| {
                let s = black_box(format!("\"{}\"", $fp::MAX));
                b.iter(move || {
                    let fp: AsString = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("serialize: f64, short", |b| {
                let fp = black_box(AsF64 {
                    v: $fp::from_bits(123456),
                });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("serialize: f64, long", |b| {
                let fp = black_box(AsF64 { v: $fp::MAX });
                let mut s = Vec::with_capacity(128);
                b.iter(move || {
                    serde_json::to_writer(&mut s, &fp).unwrap();
                    s.clear();
                })
            });

            group.bench_function("deserialize: f64, short", |b| {
                let s = black_box(format!("123456"));
                b.iter(move || {
                    let fp: AsF64 = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });

            group.bench_function("deserialize f64, long", |b| {
                let s = black_box(format!("{}", f64::from($fp::MAX) * 0.9));
                b.iter(move || {
                    let fp: AsF64 = serde_json::from_slice(s.as_bytes()).unwrap();
                    fp
                })
            });
        }
    };
}

#[cfg(feature = "i128")]
define_bench!(F128p18);
define_bench!(F64p9);

#[cfg(feature = "i128")]
criterion_group!(benches, F128p18, F64p9);
#[cfg(not(feature = "i128"))]
criterion_group!(benches, F64p9, F32p5);

criterion_main!(benches);
