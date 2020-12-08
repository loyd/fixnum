# Changelog

All notable changes to this project will be documented in this file.
`fixnum` adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.2 - 2020-12-09
### Added
- [`parity-scale-codec`][parity_scale_codec] support under the `parity` feature
  (`Encode` and `Decode` impls).
- `Into<&'static str>` was implemented for errors.

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

[parity_scale_codec]: https://docs.rs/parity-scale-codec
