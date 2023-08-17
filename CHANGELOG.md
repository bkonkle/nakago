# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0]

### Added

- Added the `nakago-sea-orm` library with an initial implementation of SeaORM Configs, Connections, and Mocked tools for testing.
- Added the `eject` operation, mostly used in testing when you want to unload the container and take ownership of referenced dependencies.
- Added a `#[Provider]` proc macro for trait implementations to automatically provide the necessary `impl Provider<Dependency>` needed for the Provider to be injected into the container.

### Changed

- Re-introduced the `<T>` type parameter to Providers, allowing various container methods to warn you before you pass a `Provider` that provides the wrong type.
- Updated tests in the Async-GraphQL package to incorporate the dependency injection system, and polished up the init flow a bit more.

## [0.7.2] - 2023-08-13

### Changed

- Re-arranged the `nakago-axum` package a bit, to remove "providers.rs" files.

## [0.7.1] - 2023-08-13

### Changed

- crossterm updated to v0.27
- sea-orm updated to v0.12

## [0.7.0] - 2023-08-13

This is a significant checkpoint, achieving fully async-driven operation with lazy Providers that are only executed when their provided Dependency is requested. To facilitate this, each Dependency is now wrapped in an Arc and no mutable references are no longer possible.

Expect major changes to the Application and Lifecycle systems going forward, building on these changes.

### Added

- Added a new `inject::Injector` type that encapsulates a new strategy for on-demand Provider execution in an async context.

### Changed

- Changes across `nakago` and `nakago-axum`, reflected in the `examples/async-graphql` example app. Providers are no longer eagerly executed, they are instead held within the injection container and are the primary way for dependencies to be provided now. When a dependency is requested, the Provider is then invoked and a [Shared Future](https://docs.rs/futures/latest/futures/future/struct.Shared.html) is created in order to share the results with any thread awaiting that dependency.

### Removed

- Removed the ability to get a mutable reference, since everything is async-by-default now and Arcs are used everywhere to allow references to traverse threads.

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

[0.8.0]: https://github.com/bkonkle/nakago/compare/0.7.2...0.8.0
[0.7.2]: https://github.com/bkonkle/nakago/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/bkonkle/nakago/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/bkonkle/nakago/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/bkonkle/nakago/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/bkonkle/nakago/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/bkonkle/nakago/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/bkonkle/nakago/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/bkonkle/nakago/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/bkonkle/nakago/releases/tag/0.1.0
