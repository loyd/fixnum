# Benchmarks

Benchmarks were performed on an [AMD Ryzen 7 4800HS CPU](https://en.wikichip.org/wiki/amd/ryzen_9/3900).

```
F128p18/cadd            time:   [2.9855 ns 2.9883 ns 2.9918 ns]
F128p18/rmul            time:   [87.376 ns 87.465 ns 87.568 ns]
F128p18/rdiv            time:   [137.72 ns 137.83 ns 137.95 ns]
F128p18/rsqrt (~10^4, precise cases)
                        time:   [376.39 ns 376.76 ns 377.16 ns]
F128p18/rsqrt (deviation)
                        time:   [1.3936 us 1.3950 us 1.3963 us]
F128p18/rsqrt (adaptive)
                        time:   [1.4963 us 1.4974 us 1.4985 us]
F128p18/next_power_of_ten
                        time:   [6.1823 ns 6.1932 ns 6.2024 ns]
F128p18/FixedPoint::try_from(f64) deviation on MIN_POSITIVE
                        time:   [1.8827 us 1.8832 us 1.8837 us]
F128p18/FixedPoint::try_from(f64) deviation on MAX
                        time:   [5.9491 us 5.9503 us 5.9516 us]
F128p18/FixedPoint::try_from(f64) ~10^-12
                        time:   [135.38 ns 135.52 ns 135.75 ns]
F128p18/FixedPoint::try_from(f64) ~0.1
                        time:   [61.967 ns 61.994 ns 62.022 ns]
F128p18/FixedPoint::try_from(f64) ~1e6
                        time:   [30.219 ns 30.259 ns 30.300 ns]
F64p9/cadd              time:   [477.73 ps 477.94 ps 478.13 ps]
F64p9/rmul              time:   [476.96 ps 477.06 ps 477.16 ps]
F64p9/rdiv              time:   [469.13 ps 469.23 ps 469.36 ps]
F64p9/rsqrt (~10^4, precise cases)
                        time:   [43.518 ns 43.545 ns 43.581 ns]
F64p9/rsqrt (deviation) time:   [96.052 ns 96.116 ns 96.172 ns]
F64p9/rsqrt (adaptive)  time:   [92.729 ns 92.893 ns 93.077 ns]
F64p9/next_power_of_ten time:   [2.6202 ns 2.6214 ns 2.6229 ns]
F64p9/FixedPoint::try_from(f64) deviation on MIN_POSITIVE
                        time:   [1.8670 us 1.8675 us 1.8682 us]
F64p9/FixedPoint::try_from(f64) deviation on MAX
                        time:   [5.9370 us 5.9462 us 5.9581 us]
F64p9/FixedPoint::try_from(f64) ~10^-12
                        time:   [135.39 ns 135.49 ns 135.60 ns]
F64p9/FixedPoint::try_from(f64) ~0.1
                        time:   [67.579 ns 67.679 ns 67.818 ns]
F64p9/FixedPoint::try_from(f64) ~1e6
                        time:   [26.862 ns 26.896 ns 26.936 ns]

F128p18/serialize 123.456 to string
                        time:   [49.152 ns 49.228 ns 49.314 ns]
F128p18/serialize MAX to string
                        time:   [73.853 ns 73.973 ns 74.108 ns]
F128p18/deserialize 123.456 from string
                        time:   [85.812 ns 85.949 ns 86.148 ns]
F128p18/deserialize MAX from string
                        time:   [318.90 ns 319.12 ns 319.35 ns]
F128p18/serialize 123.456 to f64
                        time:   [62.849 ns 62.946 ns 63.054 ns]
F128p18/serialize MAX to f64
                        time:   [67.022 ns 67.069 ns 67.117 ns]
F128p18/deserialize 123.456 from f64
                        time:   [104.12 ns 104.17 ns 104.22 ns]
F128p18/deserialize MAX from f64
                        time:   [62.896 ns 62.918 ns 62.941 ns]

F64p9/serialize 123.456 to string
                        time:   [26.397 ns 26.442 ns 26.494 ns]
F64p9/serialize MAX to string
                        time:   [33.927 ns 33.978 ns 34.031 ns]
F64p9/deserialize 123.456 from string
                        time:   [55.390 ns 55.560 ns 55.725 ns]
F64p9/deserialize MAX from string
                        time:   [87.457 ns 87.762 ns 88.064 ns]
F64p9/serialize 123.456 to f64
                        time:   [45.877 ns 45.923 ns 45.969 ns]
F64p9/serialize MAX to f64
                        time:   [40.256 ns 40.288 ns 40.319 ns]
F64p9/deserialize 123.456 from f64
                        time:   [104.09 ns 104.26 ns 104.54 ns]
F64p9/deserialize MAX from f64
                        time:   [69.204 ns 69.232 ns 69.262 ns]
```
