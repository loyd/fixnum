# Benchmarks

Benchmarks were performed on an [AMD Ryzen 7 4800HS CPU](https://en.wikichip.org/wiki/amd/ryzen_9/3900).

## ops
```
F64p9/cadd (~1e4)       time:   [1.8804 ns 1.8807 ns 1.8810 ns]
F64p9/rmul (~1e4, Floor)
                        time:   [1.8798 ns 1.8801 ns 1.8806 ns]
F64p9/rmul (~1e4, Ceil) time:   [1.9028 ns 1.9029 ns 1.9031 ns]
F64p9/rmul (~1e4, Nearest)
                        time:   [1.8928 ns 1.8937 ns 1.8951 ns]
F64p9/rdiv (~1e5/~1e4, Floor)
                        time:   [1.8809 ns 1.8815 ns 1.8824 ns]
F64p9/rdiv (~1e5/~1e4, Ceil)
                        time:   [1.8807 ns 1.8811 ns 1.8816 ns]
F64p9/rdiv (~1e5/~1e4, Nearest)
                        time:   [1.8945 ns 1.8957 ns 1.8973 ns]
F64p9/rsqrt (~1e4, Floor)
                        time:   [40.705 ns 40.774 ns 40.851 ns]
F64p9/rsqrt (~1e4, Ceil)
                        time:   [42.445 ns 42.489 ns 42.533 ns]
F64p9/rsqrt (~1e4, Nearest)
                        time:   [45.066 ns 45.111 ns 45.159 ns]
F64p9/rsqrt (MAX, Floor)
                        time:   [97.336 ns 97.454 ns 97.578 ns]
F64p9/rsqrt (MAX, Ceil) time:   [99.472 ns 99.563 ns 99.657 ns]
F64p9/rsqrt (MAX, Nearest)
                        time:   [98.629 ns 98.695 ns 98.765 ns]
F64p9/rsqrt (adaptive, Floor)
                        time:   [93.701 ns 93.768 ns 93.840 ns]
F64p9/rsqrt (adaptive, Ceil)
                        time:   [94.705 ns 94.819 ns 94.935 ns]
F64p9/rsqrt (adaptive, Nearest)
                        time:   [95.152 ns 95.240 ns 95.339 ns]
F64p9/next_power_of_ten time:   [3.4362 ns 3.4395 ns 3.4428 ns]
F64p9/FixedPoint::try_from(f64) (MIN_POSITIVE)
                        time:   [2.1977 us 2.1995 us 2.2014 us]
F64p9/FixedPoint::try_from(f64) (MAX)
                        time:   [5.9365 us 5.9373 us 5.9383 us]
F64p9/FixedPoint::try_from(f64) (~1e-12)
                        time:   [137.93 ns 137.99 ns 138.05 ns]
F64p9/FixedPoint::try_from(f64) (~0.1)
                        time:   [68.213 ns 68.275 ns 68.346 ns]
F64p9/FixedPoint::try_from(f64) (~1e6)
                        time:   [26.982 ns 27.010 ns 27.037 ns]


F128p18/cadd (~1e4)     time:   [2.8347 ns 2.8352 ns 2.8357 ns]
F128p18/rmul (~1e4, Floor)
                        time:   [132.96 ns 133.15 ns 133.39 ns]
F128p18/rmul (~1e4, Ceil)
                        time:   [131.75 ns 131.83 ns 131.91 ns]
F128p18/rmul (~1e4, Nearest)
                        time:   [132.28 ns 132.46 ns 132.65 ns]
F128p18/rdiv (~1e5/~1e4, Floor)
                        time:   [144.75 ns 144.89 ns 145.00 ns]
F128p18/rdiv (~1e5/~1e4, Ceil)
                        time:   [144.74 ns 144.84 ns 144.95 ns]
F128p18/rdiv (~1e5/~1e4, Nearest)
                        time:   [148.38 ns 148.55 ns 148.71 ns]
F128p18/rsqrt (~1e4, Floor)
                        time:   [406.28 ns 407.04 ns 407.88 ns]
F128p18/rsqrt (~1e4, Ceil)
                        time:   [446.02 ns 446.63 ns 447.23 ns]
F128p18/rsqrt (~1e4, Nearest)
                        time:   [482.21 ns 483.13 ns 484.15 ns]
F128p18/rsqrt (MAX, Floor)
                        time:   [1.3452 us 1.3465 us 1.3479 us]
F128p18/rsqrt (MAX, Ceil)
                        time:   [1.3900 us 1.3955 us 1.4019 us]
F128p18/rsqrt (MAX, Nearest)
                        time:   [1.4489 us 1.4511 us 1.4534 us]
F128p18/rsqrt (adaptive, Floor)
                        time:   [1.4559 us 1.4565 us 1.4570 us]
F128p18/rsqrt (adaptive, Ceil)
                        time:   [1.4985 us 1.4998 us 1.5011 us]
F128p18/rsqrt (adaptive, Nearest)
                        time:   [1.5441 us 1.5448 us 1.5455 us]
F128p18/next_power_of_ten
                        time:   [6.1744 ns 6.1857 ns 6.1956 ns]
F128p18/FixedPoint::try_from(f64) (MIN_POSITIVE)
                        time:   [2.1859 us 2.1884 us 2.1911 us]
F128p18/FixedPoint::try_from(f64) (MAX)
                        time:   [5.9384 us 5.9398 us 5.9416 us]
F128p18/FixedPoint::try_from(f64) (~1e-12)
                        time:   [139.05 ns 139.09 ns 139.14 ns]
F128p18/FixedPoint::try_from(f64) (~0.1)
                        time:   [61.729 ns 61.777 ns 61.826 ns]
F128p18/FixedPoint::try_from(f64) (~1e6)
                        time:   [29.475 ns 29.504 ns 29.532 ns]
```


## serde
```
F64p9/serialize 123.456 to string
                        time:   [25.126 ns 25.184 ns 25.245 ns]
F64p9/serialize MAX to string
                        time:   [34.676 ns 34.691 ns 34.708 ns]
F64p9/deserialize 123.456 from string
                        time:   [54.896 ns 55.528 ns 56.131 ns]
F64p9/deserialize MAX from string
                        time:   [83.991 ns 84.684 ns 85.462 ns]
F64p9/serialize 123.456 to f64
                        time:   [45.702 ns 45.770 ns 45.836 ns]
F64p9/serialize MAX to f64
                        time:   [41.369 ns 41.417 ns 41.489 ns]
F64p9/deserialize 123.456 from f64
                        time:   [108.28 ns 108.35 ns 108.43 ns]
F64p9/deserialize MAX from f64
                        time:   [61.488 ns 61.546 ns 61.602 ns]


F128p18/serialize 123.456 to string
                        time:   [49.650 ns 49.752 ns 49.884 ns]
F128p18/serialize MAX to string
                        time:   [72.952 ns 73.113 ns 73.279 ns]
F128p18/deserialize 123.456 from string
                        time:   [79.037 ns 79.170 ns 79.372 ns]
F128p18/deserialize MAX from string
                        time:   [184.91 ns 185.20 ns 185.49 ns]
F128p18/serialize 123.456 to f64
                        time:   [65.507 ns 65.553 ns 65.599 ns]
F128p18/serialize MAX to f64
                        time:   [67.402 ns 67.455 ns 67.509 ns]
F128p18/deserialize 123.456 from f64
                        time:   [105.50 ns 105.71 ns 106.02 ns]
F128p18/deserialize MAX from f64
                        time:   [56.141 ns 56.193 ns 56.266 ns]
```
