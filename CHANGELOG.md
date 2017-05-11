# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Added

### Changed

## [0.2.0] - 2015-12-03

### Added
- Added license information, description, etc.
- PCANBasic.def for generating a PCANBasic.lib for linking
- Added crates.io badge
- Added code to generate a `PCANBasic.lib` from the dll and def file. Requires lib.exe to be in the path.
- Added an example of bullding and linking the library

### Changed
- Changed name from pcan-basic-bindings to pcan-basic-sys
- Changed type definitions for PCANBasic.h for building on windows.

## [0.1.0] - 2017-05-04
### Added
- Started crate with cargo
- Added rustfmt.toml
- Added `PCANBasic.h`, and a basic build.rs to generated the bindings

[Unreleased]: https://github.com/cwoodall/pcan-basic-sys/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/cwoodall/pcan-basic-sys/compare/v0.1.0...v0.2.0
