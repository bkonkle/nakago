# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2023-05-08

### Added

- Added `Init`, `Startup`, and `Shutdown` lifecycle events, which Hooks can now be attached to in the renamed top-level `Application` struct (see below).
- Added `Route` specifiers to `nakago-axum` based on `Axum` routes, which are attached to the top-level `Router` via nesting.

### Changed

- Renamed `nakago::system::System` to `nakago::app::Application`
- Moved to a registry system of Hooks attached to lifecycle Events.
- Moved away from `FnvHashMap` because the keys are possibly textual rather than simple integers.
- Renamed `HttpApplication` to `AxumApplication` to better reflect the specific library used behind the scenes.
- Revised the Async-GraphQL example to use the new lifecycle events.

### Removed

- Removed most of the constructors for `Application` and `AxumApplication` (formerly `System` and `HttpApplication`) because they aren't needed now.

## [0.5.0] - 2023-02-27

### Added

- Added the `nakago-axum` crate for HTTP routes ([#9](https://github.com/bkonkle/nakago/pull/9))
- Added Config loading based on Figment ([#9](https://github.com/bkonkle/nakago/pull/9))
- Added Hooks to the inject module, which are like Providers but can mutate the Inject container ([#9](https://github.com/bkonkle/nakago/pull/9))
- Added a top-level Application with Init (pre-config) and Startup (pre-run) hooks ([#9](https://github.com/bkonkle/nakago/pull/9))
- Added an Axum HTTP Application with a `run()` method that starts the server ([#9](https://github.com/bkonkle/nakago/pull/9))
- Added a test Async-GraphQL example ([#9](https://github.com/bkonkle/nakago/pull/9))

### Changed

- Prioritized Tag-driven mode for Inject. Renamed the TypeId-driven fields to have a `_type` prefix, and removed the `_tag` prefix from the Tag-driven fields. ([#9](https://github.com/bkonkle/nakago/pull/9))

## [0.4.0] - 2023-02-15

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

[0.6.0]: https://github.com/bkonkle/nakago/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/bkonkle/nakago/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/bkonkle/nakago/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/bkonkle/nakago/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/bkonkle/nakago/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/bkonkle/nakago/releases/tag/0.1.0
