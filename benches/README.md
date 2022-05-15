# Benchmarks

Benchmarks were performed on an [AMD Ryzen 9 3900 CPU](https://en.wikichip.org/wiki/amd/ryzen_9/3900).

```
F128p18/cadd            time:   [2.7585 ns 2.7673 ns 2.7798 ns]
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) high mild
  18 (18.00%) high severe
F128p18/rmul            time:   [86.924 ns 87.552 ns 88.408 ns]
Found 16 outliers among 100 measurements (16.00%)
  14 (14.00%) low mild
  2 (2.00%) high severe
F128p18/rdiv            time:   [140.95 ns 141.06 ns 141.18 ns]
Found 30 outliers among 100 measurements (30.00%)
  2 (2.00%) low severe
  16 (16.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe
F128p18/rsqrt (~10^4, precise cases)
                        time:   [367.61 ns 368.30 ns 369.07 ns]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
F128p18/rsqrt (deviation)
                        time:   [1.3145 us 1.3166 us 1.3190 us]
F128p18/rsqrt (adaptive)
                        time:   [1.4337 us 1.4345 us 1.4352 us]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
F128p18/next_power_of_ten
                        time:   [5.7435 ns 5.7666 ns 5.7859 ns]
F128p18/FixedPoint::try_from(f64) deviation on MIN_POSITIVE
                        time:   [1.7518 us 1.7521 us 1.7524 us]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
F128p18/FixedPoint::try_from(f64) deviation on MAX
                        time:   [5.5499 us 5.5503 us 5.5507 us]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  9 (9.00%) high severe
F128p18/FixedPoint::try_from(f64) ~10^-12
                        time:   [127.28 ns 128.17 ns 129.01 ns]
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) high mild
  18 (18.00%) high severe
F128p18/FixedPoint::try_from(f64) ~0.1
                        time:   [59.345 ns 59.440 ns 59.552 ns]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
F128p18/FixedPoint::try_from(f64) ~1e6
                        time:   [29.304 ns 29.395 ns 29.500 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

F64p9/cadd              time:   [459.11 ps 459.14 ps 459.17 ps]
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) low severe
  7 (7.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe
F64p9/rmul              time:   [458.48 ps 458.52 ps 458.55 ps]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe
F64p9/rdiv              time:   [445.60 ps 445.65 ps 445.70 ps]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  7 (7.00%) high severe
F64p9/rsqrt (~10^4, precise cases)
                        time:   [40.895 ns 40.916 ns 40.937 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high severe
F64p9/rsqrt (deviation) time:   [90.416 ns 90.510 ns 90.606 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
F64p9/rsqrt (adaptive)  time:   [87.880 ns 87.927 ns 87.975 ns]
Found 22 outliers among 100 measurements (22.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  20 (20.00%) high severe
F64p9/next_power_of_ten time:   [2.4977 ns 2.5153 ns 2.5343 ns]
F64p9/FixedPoint::try_from(f64) deviation on MIN_POSITIVE
                        time:   [1.7986 us 1.7988 us 1.7990 us]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe
F64p9/FixedPoint::try_from(f64) deviation on MAX
                        time:   [5.5439 us 5.5444 us 5.5450 us]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
F64p9/FixedPoint::try_from(f64) ~10^-12
                        time:   [133.16 ns 133.24 ns 133.31 ns]
F64p9/FixedPoint::try_from(f64) ~0.1
                        time:   [63.531 ns 63.580 ns 63.637 ns]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
F64p9/FixedPoint::try_from(f64) ~1e6
                        time:   [25.607 ns 25.623 ns 25.639 ns]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

F128p18/serialize: string, short
                        time:   [162.76 ns 162.88 ns 162.99 ns]
Found 16 outliers among 100 measurements (16.00%)
  10 (10.00%) high mild
  6 (6.00%) high severe
F128p18/serialize: string, long
                        time:   [135.34 ns 135.77 ns 136.20 ns]
F128p18/deserialize: string, short
                        time:   [77.147 ns 77.177 ns 77.209 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
F128p18/deserialize string, long
                        time:   [310.69 ns 310.87 ns 311.05 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
F128p18/serialize: f64, short
                        time:   [53.129 ns 53.185 ns 53.236 ns]
Found 22 outliers among 100 measurements (22.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  16 (16.00%) high severe
F128p18/serialize: f64, long
                        time:   [65.015 ns 65.059 ns 65.105 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
F128p18/deserialize: f64, short
                        time:   [37.621 ns 37.627 ns 37.633 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
F128p18/deserialize f64, long
                        time:   [60.117 ns 60.154 ns 60.193 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

F64p9/serialize: string, short
                        time:   [103.53 ns 103.65 ns 103.81 ns]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
F64p9/serialize: string, long
                        time:   [84.568 ns 84.621 ns 84.677 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
F64p9/deserialize: string, short
                        time:   [37.751 ns 37.838 ns 37.926 ns]
F64p9/deserialize string, long
                        time:   [82.921 ns 83.373 ns 83.836 ns]
F64p9/serialize: f64, short
                        time:   [44.441 ns 44.458 ns 44.476 ns]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe
F64p9/serialize: f64, long
                        time:   [40.707 ns 40.743 ns 40.775 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
F64p9/deserialize: f64, short
                        time:   [30.811 ns 30.820 ns 30.830 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe
F64p9/deserialize f64, long
                        time:   [66.514 ns 66.565 ns 66.616 ns]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```
