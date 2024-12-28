# Benchmarks

Benchmarks were performed on an Intel Core i9-14900K CPU.

```sh
$ cargo bench --bench <name> --features <int>
$ critcmp new | tail +3 | sort | sed 's#        ? ?/sec##' | sed 's#    1.00##'
```

## ops
64-bit FP with precision = 9:
```
F64p9/cadd (~1e4)                          1.0±0.03ns
F64p9/from_decimal(12345, -3)              1.0±0.01ns
F64p9/next_power_of_ten                    1.6±0.03ns
F64p9/rdiv (~1e5/~1e4, Ceil)               1.0±0.03ns
F64p9/rdiv (~1e5/~1e4, Floor)              1.0±0.04ns
F64p9/rdiv (~1e5/~1e4, Nearest)            1.0±0.04ns
F64p9/rmul (~1e4, Ceil)                    1.0±0.03ns
F64p9/rmul (~1e4, Floor)                   1.0±0.04ns
F64p9/rmul (~1e4, Nearest)                 1.0±0.05ns
F64p9/rsqrt (~1e4, Ceil)                   1.0±0.02ns
F64p9/rsqrt (~1e4, Floor)                  1.0±0.02ns
F64p9/rsqrt (~1e4, Nearest)                1.0±0.03ns
F64p9/rsqrt (adaptive, Ceil)               5.4±0.02ns
F64p9/rsqrt (adaptive, Floor)              4.9±0.01ns
F64p9/rsqrt (adaptive, Nearest)            5.5±0.02ns
F64p9/rsqrt (MAX, Ceil)                    1.0±0.01ns
F64p9/rsqrt (MAX, Floor)                   1.0±0.01ns
F64p9/rsqrt (MAX, Nearest)                 1.0±0.01ns
F64p9/to_decimal(0) (12.345)               5.0±0.01ns
F64p9/to_decimal(i32::MAX) (12.345)        5.0±0.02ns
F64p9/try_from(f64) (~0.1)                33.2±0.08ns
F64p9/try_from(f64) (~1e-12)              61.9±0.20ns
F64p9/try_from(f64) (~1e6)                16.2±0.05ns
F64p9/try_from(f64) (MAX)               1263.8±2.26ns
F64p9/try_from(f64) (MIN_POSITIVE)       693.4±2.38ns
```

128-bit FP with precision = 18:
```
F128p18/cadd (~1e4)                        1.9±0.05ns
F128p18/from_decimal(12345, -3)            4.8±0.02ns
F128p18/next_power_of_ten                  3.1±0.04ns
F128p18/rdiv (~1e5/~1e4, Ceil)            10.7±0.15ns
F128p18/rdiv (~1e5/~1e4, Floor)           10.4±0.15ns
F128p18/rdiv (~1e5/~1e4, Nearest)         11.2±0.16ns
F128p18/rmul (~1e4, Ceil)                  7.0±0.04ns
F128p18/rmul (~1e4, Floor)                 7.0±0.02ns
F128p18/rmul (~1e4, Nearest)               7.2±0.06ns
F128p18/rsqrt (~1e4, Ceil)                40.0±0.24ns
F128p18/rsqrt (~1e4, Floor)               39.4±0.28ns
F128p18/rsqrt (~1e4, Nearest)             41.2±0.28ns
F128p18/rsqrt (adaptive, Ceil)            50.0±0.42ns
F128p18/rsqrt (adaptive, Floor)           49.2±0.42ns
F128p18/rsqrt (adaptive, Nearest)         50.6±0.38ns
F128p18/rsqrt (MAX, Ceil)                 40.2±0.28ns
F128p18/rsqrt (MAX, Floor)                39.3±0.27ns
F128p18/rsqrt (MAX, Nearest)              41.4±0.38ns
F128p18/to_decimal(0) (12.345)            59.1±0.19ns
F128p18/to_decimal(i32::MAX) (12.345)     59.1±0.28ns
F128p18/try_from(f64) (~0.1)              28.5±1.51ns
F128p18/try_from(f64) (~1e-12)            62.1±0.20ns
F128p18/try_from(f64) (~1e6)              15.2±0.04ns
F128p18/try_from(f64) (MAX)             1264.6±4.34ns
F128p18/try_from(f64) (MIN_POSITIVE)     693.6±2.45ns
```

## serde
64-bit FP with precision = 9:
```
F64p9/deserialize 123.456 from f64          55.4±0.17ns
F64p9/deserialize 123.456 from string       27.1±0.34ns
F64p9/deserialize MAX from f64              44.4±0.03ns
F64p9/deserialize MAX from string           39.3±0.61ns
F64p9/serialize 123.456 to f64              27.0±0.33ns
F64p9/serialize 123.456 to string           13.1±0.21ns
F64p9/serialize MAX to f64                  38.6±0.01ns
F64p9/serialize MAX to string               14.8±0.19ns
```

128-bit FP with precision = 18:
```
F128p18/deserialize 123.456 from f64        55.9±0.07ns
F128p18/deserialize 123.456 from string     31.5±0.74ns
F128p18/deserialize MAX from f64            40.8±0.20ns
F128p18/deserialize MAX from string         60.1±0.75ns
F128p18/serialize 123.456 to f64            30.4±0.15ns
F128p18/serialize 123.456 to string         23.6±0.29ns
F128p18/serialize MAX to f64                23.4±0.02ns
F128p18/serialize MAX to string             37.3±0.04ns
```
