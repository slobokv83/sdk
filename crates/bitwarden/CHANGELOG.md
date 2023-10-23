# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2023-10-13

### Changed

- `auth::request::AccessTokenLoginRequest` moved to `auth::login::AccessTokenLoginRequest` (#178)
- Support for fetching multiple secrets by ids (#150)

## [0.3.0] - 2023-07-26

### Deprecated

- The secrets manager SDK is now hidden behind a `secrets` feature flag. Make sure to enable this
  flag in your `Cargo.toml` file. At the moment the flag is enabled by default for compatibility
  reasons, but this is considered deprecated and the flag will be made opt-in eventually.

### Added

- Support for creating and editing secrets (#77)
- Support for creating and editing projects (#53)

### Changed

- Folder structure, update `use` declarations (#68)

### Fixed

- Improve login error handling (#109)

## [0.2.1] - 2023-03-22

### Fixed

- Add user agent to login requests (#11)
