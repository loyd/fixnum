# Benchmarks

Benchmarks were performed on an [AMD Ryzen 9 3900 CPU](https://en.wikichip.org/wiki/amd/ryzen_9/3900).

```
F128p18/cadd            time:   [2.7240 ns 2.7260 ns 2.7284 ns]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
F128p18/rmul            time:   [75.851 ns 76.105 ns 76.381 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe
F128p18/rdiv            time:   [131.22 ns 131.33 ns 131.45 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
F128p18/rsqrt (~10^4, precise cases)
                        time:   [406.19 ns 406.52 ns 406.89 ns]
F128p18/rsqrt (deviation)
                        time:   [1.6058 us 1.6072 us 1.6088 us]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
F128p18/rsqrt (adaptive)
                        time:   [1.7371 us 1.7380 us 1.7389 us]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
F128p18/next_power_of_ten
                        time:   [5.8670 ns 5.8787 ns 5.8895 ns]

F64p9/cadd              time:   [457.84 ps 458.13 ps 458.46 ps]
F64p9/rmul              time:   [455.02 ps 455.34 ps 455.70 ps]
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high mild
F64p9/rdiv              time:   [437.68 ps 438.00 ps 438.36 ps]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
F64p9/rsqrt (~10^4, precise cases)
                        time:   [38.846 ns 38.885 ns 38.926 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
F64p9/rsqrt (deviation) time:   [93.532 ns 93.651 ns 93.764 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
F64p9/rsqrt (adaptive)  time:   [89.743 ns 89.795 ns 89.839 ns]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  6 (6.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
F64p9/next_power_of_ten time:   [2.5555 ns 2.5574 ns 2.5595 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild

F32p5/cadd              time:   [228.09 ps 228.17 ps 228.25 ps]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
F32p5/rmul              time:   [230.57 ps 230.77 ps 230.97 ps]
F32p5/rdiv              time:   [222.68 ps 222.95 ps 223.24 ps]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
F32p5/rsqrt (~10^4, precise cases)
                        time:   [5.7505 ns 5.7534 ns 5.7564 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
F32p5/rsqrt (deviation) time:   [5.7533 ns 5.7571 ns 5.7618 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
F32p5/rsqrt (adaptive)  time:   [5.7865 ns 5.7926 ns 5.8004 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
F32p5/next_power_of_ten time:   [3.4495 ns 3.4520 ns 3.4547 ns]
```
