# Benchmarks

Benchmarks were performed on an [AMD Ryzen 7 4800HS CPU](https://en.wikichip.org/wiki/amd/ryzen_9/3900).

```sh
$ cargo bench --bench <name> --features <int>
$ critcmp new | tail +3 | sort | sed 's#        ? ?/sec##'
```

## ops
64-bit FP with precision = 9:
```
F64p9/cadd (~1e4)                        1.00      1.9±0.01ns
F64p9/from_decimal(12345, -3)            1.00      1.6±0.00ns
F64p9/next_power_of_ten                  1.00      3.6±0.01ns
F64p9/rdiv (~1e5/~1e4, Ceil)             1.00      1.9±0.01ns
F64p9/rdiv (~1e5/~1e4, Floor)            1.00      1.9±0.01ns
F64p9/rdiv (~1e5/~1e4, Nearest)          1.00      1.9±0.00ns
F64p9/rmul (~1e4, Ceil)                  1.00      1.9±0.01ns
F64p9/rmul (~1e4, Floor)                 1.00      1.9±0.03ns
F64p9/rmul (~1e4, Nearest)               1.00      1.9±0.00ns
F64p9/rsqrt (~1e4, Ceil)                 1.00     43.7±0.29ns
F64p9/rsqrt (~1e4, Floor)                1.00     42.5±0.17ns
F64p9/rsqrt (~1e4, Nearest)              1.00     47.0±0.19ns
F64p9/rsqrt (adaptive, Ceil)             1.00     98.0±0.33ns
F64p9/rsqrt (adaptive, Floor)            1.00     94.4±1.45ns
F64p9/rsqrt (adaptive, Nearest)          1.00     99.6±0.67ns
F64p9/rsqrt (MAX, Ceil)                  1.00    102.3±0.50ns
F64p9/rsqrt (MAX, Floor)                 1.00    100.2±0.50ns
F64p9/rsqrt (MAX, Nearest)               1.00    102.7±0.80ns
F64p9/to_decimal(0) (12.345)             1.00      9.1±0.02ns
F64p9/to_decimal(i32::MAX) (12.345)      1.00      9.1±0.01ns
F64p9/try_from(f64) (~0.1)               1.00     64.8±0.33ns
F64p9/try_from(f64) (~1e-12)             1.00    132.5±0.46ns
F64p9/try_from(f64) (~1e6)               1.00     24.9±0.14ns
F64p9/try_from(f64) (MAX)                1.00      5.9±0.01µs
F64p9/try_from(f64) (MIN_POSITIVE)       1.00   1872.9±4.12ns
```

128-bit FP with precision = 18:
```
F128p18/cadd (~1e4)                      1.00      2.8±0.00ns
F128p18/from_decimal(12345, -3)          1.00      9.1±0.03ns
F128p18/next_power_of_ten                1.00      6.3±0.03ns
F128p18/rdiv (~1e5/~1e4, Ceil)           1.00    157.3±0.51ns
F128p18/rdiv (~1e5/~1e4, Floor)          1.00    154.2±1.19ns
F128p18/rdiv (~1e5/~1e4, Nearest)        1.00    159.4±1.05ns
F128p18/rmul (~1e4, Ceil)                1.00    132.5±0.61ns
F128p18/rmul (~1e4, Floor)               1.00    132.3±0.79ns
F128p18/rmul (~1e4, Nearest)             1.00    134.1±0.79ns
F128p18/rsqrt (~1e4, Ceil)               1.00    428.3±7.08ns
F128p18/rsqrt (~1e4, Floor)              1.00    403.9±1.24ns
F128p18/rsqrt (~1e4, Nearest)            1.00    475.3±1.03ns
F128p18/rsqrt (adaptive, Ceil)           1.00   1469.3±3.05ns
F128p18/rsqrt (adaptive, Floor)          1.00   1436.2±1.98ns
F128p18/rsqrt (adaptive, Nearest)        1.00   1530.6±1.97ns
F128p18/rsqrt (MAX, Ceil)                1.00   1393.2±9.68ns
F128p18/rsqrt (MAX, Floor)               1.00  1335.9±10.01ns
F128p18/rsqrt (MAX, Nearest)             1.00  1441.7±11.63ns
F128p18/to_decimal(0) (12.345)           1.00   263.8±25.35ns
F128p18/to_decimal(i32::MAX) (12.345)    1.00    263.2±0.13ns
F128p18/try_from(f64) (~0.1)             1.00     59.3±0.36ns
F128p18/try_from(f64) (~1e-12)           1.00    133.0±0.14ns
F128p18/try_from(f64) (~1e6)             1.00     27.8±0.25ns
F128p18/try_from(f64) (MAX)              1.00      5.9±0.00µs
F128p18/try_from(f64) (MIN_POSITIVE)     1.00   1842.6±1.86ns
```

## serde
64-bit FP with precision = 9:
```
F64p9/deserialize 123.456 from f64         1.00    103.7±0.24ns
F64p9/deserialize 123.456 from string      1.00     54.8±0.18ns
F64p9/deserialize MAX from f64             1.00     59.8±0.24ns
F64p9/deserialize MAX from string          1.00     86.3±0.79ns
F64p9/serialize 123.456 to f64             1.00     48.2±0.46ns
F64p9/serialize 123.456 to string          1.00     27.5±0.29ns
F64p9/serialize MAX to f64                 1.00     41.3±0.95ns
F64p9/serialize MAX to string              1.00     35.3±2.63ns
```

128-bit FP with precision = 18:
```
F128p18/deserialize 123.456 from f64       1.00    103.3±0.24ns
F128p18/deserialize 123.456 from string    1.00     70.8±0.09ns
F128p18/deserialize MAX from f64           1.00     56.6±0.19ns
F128p18/deserialize MAX from string        1.00    147.3±0.51ns
F128p18/serialize 123.456 to f64           1.00     67.7±0.38ns
F128p18/serialize 123.456 to string        1.00     51.7±0.64ns
F128p18/serialize MAX to f64               1.00     63.6±0.74ns
F128p18/serialize MAX to string            1.00     80.6±1.00ns
```
