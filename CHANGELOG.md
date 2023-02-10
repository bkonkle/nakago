# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-02-10

### Added

- Add `inject::ProvideResult<dyn Repository>` and `inject::provide(...)` to make the `Ok(Box::new(...))` less obtrusive. ([#3](https://github.com/bkonkle/nakago/pull/3))

### Updated

- Additional injection documentation

## [0.1.0] - 2023-02-09

### Added

- An Injection Container
- Injection Tags
- Injection Providers
- Documentation

[0.2.0]: https://github.com/bkonkle/nakago/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/bkonkle/nakago/releases/tag/0.1.0
