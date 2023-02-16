# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Moved the "tag" parameter on all tag-based operations to the front of the method signature, for easier readability. ([#8](https://github.com/bkonkle/nakago/pull/8))

## [0.3.0] - 2023-02-15

### Changed

- Export more things for easier ergonomics ([#5](https://github.com/bkonkle/nakago/pull/5))

### Fixed

- Fixed the CI build ([#4](https://github.com/bkonkle/nakago/pull/4))
- Disabled auto-publishing (for now) ([#4](https://github.com/bkonkle/nakago/pull/4))

### Removed

- Remove `inject::ProvideResult` and `inject::provide(...)` and clean up unnecessary boxing. ([#6](https://github.com/bkonkle/nakago/pull/6))

## [0.2.0] - 2023-02-10

### Added

- Add `inject::ProvideResult` and `inject::provide(...)` to make the `Ok(Box::new(...))` less obtrusive. ([#3](https://github.com/bkonkle/nakago/pull/3))

### Changed

- Additional injection documentation

## [0.1.0] - 2023-02-09

### Added

- An Injection Container
- Injection Tags
- Injection Providers
- Documentation

[0.3.0]: https://github.com/bkonkle/nakago/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/bkonkle/nakago/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/bkonkle/nakago/releases/tag/0.1.0
