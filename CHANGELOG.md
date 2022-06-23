# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate
### Added
- A new round mode: `RoundMode::Nearest`.

### Changed
- Replace the `RoundingSqrt` trait with the `rsqrt` method.

### Removed
- Deprecated methods `rounding_from_f64` and `to_f64`. Use `TryFrom<f64>` and `Into<f64>` instead.

## [0.7.0] - 2022-05-15
### Fixed
- Rare "from `f64`" bugs ([#25]).

### Changed
- Update `parity-scale-codec` to `v3` ([#26]).
- Optimize `From<f64>` instance ([#25]).
- Optimize serialization to string and `Display` instance ([#29]).

[#25]: https://github.com/loyd/fixnum/pull/25
[#26]: https://github.com/loyd/fixnum/issues/26
[#29]: https://github.com/loyd/fixnum/pull/29

## [0.6.1] - 2021-12-30
### Changed
- serde: can be used in no_std environments ([#24]).
- `TryFrom<f64>` can be used in no_std environments ([#22]).

[#24]: https://github.com/loyd/fixnum/pull/24
[#22]: https://github.com/loyd/fixnum/pull/22

## [0.6.0] - 2021-07-01
### Added
- `serde::as_string`, `serde::as_float`, `serde::as_repr`.
- `i64`, `i32`, `i16` features.

### Fixed
- To/from f64 conversion ([#25]).

### Changed
- No implementation is provided by default, use `i64` or `i128` features.
- (De)serialize `FixedPoint` as a string in case of human readable formats.
- `ConvertError` and `FromDecimalError` are merged into one opaque `ConvertError` type.

[#25]: https://github.com/loyd/fixnum/pull/25

## [0.5.1] - 2021-05-28
### Added
- docs: specify required features for feature-gated instances.

## [0.5.0] - 2021-03-24
### Added
- Trait `ops::RoundingSqrt` and its implementation for `FixedPoint` and `int`s ([#17]).
- `ArithmeticError::DomainViolation` error for attempts to compute a square root of a negative number ([#17]).

### Changed
- `FixedPoint::half_sum` now takes `RoundMode` parameter ([#17]).

[#17]: https://github.com/loyd/fixnum/pull/17

## [0.4.0] - 2021-03-05
### Changed
- Update `parity-scale-codec` to `v2` ([#16]).

[#16]: https://github.com/loyd/fixnum/pull/16

## [0.3.1] - 2021-02-16
### Added
- Method `FixedPoint::into_bits` ([#14]).
- More thorough feature testing ([#13]).
- "Compile fail" test for `fixnum_const!` macro ([#13]).

### Changed
- Unit tests for default fixnum feature set and `i128` feature were unified ([#13]).

[#14]: https://github.com/loyd/fixnum/pull/14
[#13]: https://github.com/loyd/fixnum/pull/13

## [0.3.0] - 2021-01-25
### Removed
- Support of `fixnum::ops::Numeric` ([#10]).

### Added
- Traits `ops::One`, `ops::Zero`, `ops::Bounded`.
- Saturating operations ([#10]):
  - `CheckedAdd::saturating_add`,
  - `CheckedMul::saturating_mul`,
  - `CheckedSub::saturating_sub`,
  - `RoundingMul::saturating_rmul`.

[#10]: https://github.com/loyd/fixnum/pull/10

## [0.2.3] - 2020-12-30
### Added
- Const fixed-point literal macro `fixnum_const!` ([#11]).
- `Compact` implementation for `parity` feature ([#9]).
- `Clone` implementation for errors ([#9]).

### Fixed
- `parity`'s `Encode` and `Decode` impls ([#9]).

[#11]: https://github.com/loyd/fixnum/pull/11
[#9]: https://github.com/loyd/fixnum/pull/9

## [0.2.2] - 2020-12-09
### Added
- [`parity-scale-codec`](https://docs.rs/parity-scale-codec) support under the `parity` feature: `Encode` and `Decode` instances ([#8]).
- `fixnum!` macro for compile-time-checked fixed-point "literals".
- `.as_str()` was implemented for errors ([#8]).

### Changed
- Added `$crate` prefix for `impl_op` macro ([#8]).

[#8]: https://github.com/loyd/fixnum/pull/8

## [0.2.1] - 2020-12-04
### Changed
- Docs' links were fixed ([#6]).

[#6]: https://github.com/loyd/fixnum/pull/6

## [0.2.0] - 2020-12-04
### Removed
- Support of `fixnum::ops::CheckedDiv` ([#5]).

### Changed
- There's no need in `uint` crate anymore. This significantly helps in `no_std` environments ([#5]).

[#5]: https://github.com/loyd/fixnum/pull/5

## [0.1.0] - 2020-12-03
### Added
- Initial release.

<!-- next-url -->
[Unreleased]: https://github.com/loyd/fixnum/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/loyd/fixnum/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/loyd/fixnum/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/loyd/fixnum/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/loyd/fixnum/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/loyd/fixnum/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/loyd/fixnum/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/loyd/fixnum/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/loyd/fixnum/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/loyd/fixnum/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/loyd/fixnum/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/loyd/fixnum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/loyd/fixnum/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/loyd/fixnum/releases/tag/v0.1.0
