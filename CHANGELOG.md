# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.3] - 2021-12-09
### Added
- Explicit bound `Self: 'long` in reference upcast functions (`upcast_item_ref`, `upcast_item_mut`, `upcast_key_ref`).
  This is required by the latest version of Rust.

## [0.7.2] - 2021-12-01
### Added
- Impl `GetKeyValue` for `serde_json::Map<String, serde_json::Value>`.

## [0.7.1] - 2021-11-17
### Added 
- `GetKeyValue` and `GetKeyValueMut`.
  Implementations for `HashMap`, `BTreeMap` and `ijson::Object`.

## [0.6.0] - 2021-10-07
### Added 
- Required `Clone` traits on immutable reference types.

## [0.5.2] - 2021-09-26
### Added
- `Get` and `GetMut` impls for `SmallVec`.

## [0.5.1] - 2021-09-26
### Changed
- Fix features `slab` and `smallvec`.

## [0.5.0] - 2021-09-22 [YANKED]
### Yanking reason
- Errors with features `slab` and `smallvec`.

### Added 
- `Keyed` and `KeyedRef` traits.