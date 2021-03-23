# Changelog

All notable changes to this project will be documented in this file.
`fixnum` adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2021-03-24
### Added
- Trait `ops::RoundingSqrt` and its implementation for `FixedPoint` and `int`s (@quasiyoke).
- `ArithmeticError::DomainViolation` error for attempts to compute a square root of a negative number (@quasiyoke).

### Changed
- `FixedPoint::half_sum` now takes `RoundMode` parameter (@quasiyoke).

## [0.4.0] - 2021-03-05
### Changed
- `parity-scale-codec v2` (@KalitaAlexey).

## [0.3.1] - 2021-02-16
### Added
- Method `FixedPoint::into_bits` (@KalitaAlexey).
- More thorough feature testing (@quasiyoke).
- "Compile fail" test for `fixnum_const!` macro (@quasiyoke).

### Changed
- Unit tests for default fixnum feature set and `i128` feature were unified (@quasiyoke).

## [0.3.0] - 2021-01-25
### Removed
- Support of `fixnum::ops::Numeric` (@quasiyoke).

### Added
- Traits `ops::One`, `ops::Zero`, `ops::Bounded`.
- Saturating operations (@quasiyoke):
  - `CheckedAdd::saturating_add`,
  - `CheckedMul::saturating_mul`,
  - `CheckedSub::saturating_sub`,
  - `RoundingMul::saturating_rmul`.
- Docs for `FixedPoint::integral` method (@quasiyoke).

## [0.2.3] - 2020-12-30
### Added
- Const fixed-point literal macro `fixnum_const!` (@quasiyoke).
- `Compact` implementation for `parity` feature (@quasiyoke).
- `Clone` implementation for errors (@quasiyoke).

### Fixed
- `parity`'s `Encode` and `Decode` impls (@quasiyoke).

## [0.2.2] - 2020-12-09
### Added
- [`parity-scale-codec`](https://docs.rs/parity-scale-codec) support under the `parity` feature
  (`Encode` and `Decode` impls; @quasiyoke).
- `fixnum!` macro for compile-time-checked fixed-point "literals".
- `.as_str()` was implemented for errors (@quasiyoke).

### Changed
- Added `$crate` prefix for `impl_op` macro (@quasiyoke).

## [0.2.1] - 2020-12-04
### Changed
- Docs' links were fixed (@quasiyoke).

## [0.2.0] - 2020-12-04
### Removed
- Support of `fixnum::ops::CheckedDiv` (@quasiyoke).

### Changed
- There's no need in `uint` crate anymore. This significantly helps in `no_std` environments (@quasiyoke).

## [0.1.0] - 2020-12-03
### Added
- Initial release.
