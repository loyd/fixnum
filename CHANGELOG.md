# Changelog

All notable changes to this project will be documented in this file.
`fixnum` adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

## 0.2.3 - 2020-12-30
### Added
- `Compact` implementation for `parity` feature (@quasiyoke).
- `Clone` implementation for errors (@quasiyoke).

### Fixed
- `parity`'s `Encode` and `Decode` impls (@quasiyoke).

## 0.2.2 - 2020-12-09
### Added
- [`parity-scale-codec`](https://docs.rs/parity-scale-codec) support under the `parity` feature
  (`Encode` and `Decode` impls; @quasiyoke).
- `fixnum!` macro for compile-time-checked fixed-point "literals".
- `.as_str()` was implemented for errors (@quasiyoke).

### Changed
- Added `$crate` prefix for `impl_op` macro (@quasiyoke).

## 0.2.1 - 2020-12-04
### Changed
- Docs' links were fixed (@quasiyoke).

## 0.2.0 - 2020-12-04
### Removed
- Support of `fixnum::ops::CheckedDiv` (@quasiyoke).

### Changed
- There's no need in `uint` crate anymore. This significantly helps in `no_std` environments (@quasiyoke).

## 0.1.0 - 2020-12-03
### Added
- Initial release.
