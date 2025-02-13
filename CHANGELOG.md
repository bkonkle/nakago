# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.25.0]

### Changed

- `nakago-axum` - Upgrade Axum to v0.8, with breaking changes.
- `nakago-ws` - Upgrade Axum to v0.8, with breaking changes.
- `nakago-warp` - Upgrade tower-http to v0.6
- `nakago-async-graphql` - Upgrade Axum to v0.8 and async-graphql to a git branch for compatibility.
- _(all packages)_ - Other minor dependency updates.

## [0.24.1]

### Changed

- _(all packages)_ - Remove the `git = "<https://github.com/cksac/fake-rs.git>"` parameter from the `fake` dependency, as it is no longer needed.
- `nakago-axum` - Change `std::panic::PanicInfo` to `std::panic::PanicHookInfo`

## [0.24.0]

### Added

- `nakago-axum` - Added a `Token<T>` extractor that uses the `T` for private claims using biscuit::ClaimsSet.

### Removed

- `nakago-ws` - Removed `Session` in favor of a generic type parameter for the `Connections` struct.

### Changed

- `nakago-axum` - Renamed the auth `Validator` to `JWKSValidator` for clarity.
- `nakago-axum` - Moved the Unverified Validator out to a separate struct that is only built when `cfg(test)` is enabled.
- `nakago-ws` - Handlers now accept a new Token type, which contains the JWT string and the Registered Claims for use in more complex authorization scenarios.
- `nakago-ws` - Changed the signature of Handlers to use Results for better error handling.
- `nakago-async-graphql` - Updated `nakago-axum`.

## [0.23.0]

Major pivot! Until now, Nakago has been aimed at growing into a full-scale API server framework. However, I've realized that the core value of Nakago is the Dependency Injection system, and that it can be used in a wide variety of contexts. This release is a pivot to focus on the DI system itself, and to make it more flexible and easier to use in a variety of contexts.

### Removed

- `nakago` - Removed `Application`, `Hooks`, the `lifecycle::EventType` and `lifecycle::Events` enums. The `config` utilities were moved to `nakago-figment`.
- `nakago-axum` - Removed `AxumApplication`, `Route`, and `Routes` in favor of a more generic approach. Use the new `Inject` extractor to access dependencies in your Axum handlers.
- `nakago-warp` - Removed `WarpApplication`, `Route`, and `Routes` in favor of a more generic approach. Use the `with_injection()` Filter to access dependencies in your Warp handlers.

### Changed

- `nakago` - The tag-based container operations were previously the default mode of working with `Inject`. This proved to be more verbose and than necessary in most cases, however. The new default mode is to use the type-based operations, which are more ergonomic and easier to use. The tag-based operations are still available, but they are now suffixed with `_tag` to make it clear that they are a different mode of operation. The type-based operations are no longer suffixed with `_type`, because they are now the default.
- `nakago` - The `inject` namespace has been promoted to the top level. Where `nakago::inject` was used before, `nakago` should now be the equivalent.
- `nakago` - The `config` utilities were moved to `nakago-figment`.
- `nakago-examples-simple`, `nakago-examples-async-graphql`, `nakago-examples-simple-warp` - Updated with the new approach. See these examples for more information about how to use the tools mentioned above.

### Added

- `nakago-axum` - Added a new `Inject` extractor to access dependencies in your Axum handlers. This is a much more idiomatic way to use Nakago with Axum, and it's more flexible than the previous approach. Use the provided Axum `State` to make the `Inject` container available to your handlers.
- `nakago-figment` - Generic config utilities extracted from the core `nakago` package.

## [0.22.0]

### Changed

- `nakago-sea-orm` - Removed the argument from `nakago_sea_orm::connection::Provide::new()`, because there is the with_config_tag chained helper to provide that.

## [0.21.0]

### Changed

- `nakago-axum` - Changed the `Inject` extractor to be a wrapper around the Nakago dependency injection container, rather than a direct extractor. This makes it much more convenient to use it in an idiomatic way with Axum.
- `nakago-async-graphql` - Updated `nakago-axum`
- `nakago-ws` - Updated `nakago-axum`
- `nakago-examples-async-graphql` - Updated the example to use the new `Inject` extractor.

## [0.20.0]

### Added

- `nakago-ws`: New package for Websocket utilities

### Changed

- `nakago`: Major change - types are now the primary interface, with tags being used only when needed.
  - Tags introduced more boilerplate than I liked, and they're really only necessary when you're dealing with multiple instances of the same type. This change makes the API more ergonomic and easier to understand.
  - Operations like `.get_type()` and `.provide_type()` have been moved to methods like `.get()` and `.provide()`, and the existing tag methods were moved to methods like `.get_tag()` and `.provide_tag()`.
- `nakago-examples-async-graphql`: Updated to use the new `nakago-ws` package for Websocket support.

### Fixed

- `nakago-derive`: Properly namespaced the `Inject` usage

## [0.19.1]

### Changed

- `nakago-example-async-graphql`: Cleaned up some imports that weren't being used.
- Removed the 'config' directories in the example projects and moved the config files up to the root folder of each project.
- Updated config loaders to act on Figments, making it easier to take full advantage of the Figment library.

## [0.19.0]

### Added

- `nakago-warp`: A new Warp adapter that works in a similar way to the Axum adapter.
- `nakago-examples-simple-warp`: A new example project that uses the Warp adapter.
- `nakago`: Added a copy of Axum's `FromRef` utility, so that it can be used without importing Axum itself.
- `nakago-derive`: Updated to support the FromRef utility.

### Changed

- Updated `mockall` and `tokio-tungstenite` requirements, and removed temporary tokio-tungstenite fork.
- `nakago-axum`: Simplified the route Init Hook.
- `nakago-axum`, `nakago-async-graphql`, `nakago-sea-orm`: Updated to use the new FromRef utility.

## [0.18.0]

### Changed

- `nakago`: Errors were split into 3 - one for Injection, one for Providers, and one for Hooks.
- `nakago-derive`: Updated to use the updated error type for Providers.

## [0.17.0]

This is a big release! It includes updates from `http` to v1.0, `hyper` to v1.0, and `axum` to v0.7. It refactors the test utils to use `reqwest` instead of the minimal hyper Client that is changing in v1.0. There are a number of small behind-the-scenes changes to support these new versions, and Nakago is currently relying on a few temporary forks as the community catches up to the big releases in recent weeks.

### Changed

- Dependency updates to `http` v1.0, `hyper` v1.0, and `axum` v0.7 across the board.
- `nakago-axum`: `AxumApplication.run()` now returns a tuple with the server and the actual address that it is bound to.
- `nakago-axum`: The `jwks` utils now use `reqwest` to retrieve the "known-hosts.json" file.
- `nakago-axum`: The `Route` and `Routes` types no longer take a `Body` type parameter.
- `nakago-axum`: Moved the test HTTP utilities to `reqwest`, eliminating the need for an injected HTTP test client.
- `nakago-async-graphql`: Moved the test HTTP utilities to `reqwest`, and made the integration with the plain HTTP test utils more seamless.

### Removed

- `nakago-axum`: Removed the `test::CLIENT` tag, because it is no longer needed.

### Upgrade Guide

To update your project from Nakago v0.16 to v0.17:

- Update all Nakago crates to v0.17.

- Update usages of `AxumApplication.run()` to use the new return type, which is a tuple of `(Server, SocketAddr)`.

```rust
    // From:
    let server = app.run(args.config_path).await?;
    let addr = server.local_addr();

    // To:
    let (server, addr) = app.run(args.config_path).await?;
```

- Update tests to use `reqwest` instead of the injected HTTP client.

```rust
    // From:
    let req = utils
        .http
        .call(Method::GET, "/username", Value::Null, Some(&token))?;

    let resp = utils.http_client.request(req).await?;

    let status = resp.status();
    let body = to_bytes(resp.into_body()).await?;

    let json: Value = serde_json::from_slice(&body)?;

    // To:
    let resp = utils
        .http
        .get_json("/username", Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;
```


## [0.16.0]

### Added

- `tutorial`: More revisions to the tutorial documentation.

### Changed

- `nakago-axum`: Renamed the `auth::subject::Provide` provider for `Validator` - which didn't make sense - to `validator::Provide`.

## [0.15.0]

### Removed

- `nakago-async-graphql`: Removed the generic SchemaBuilder, since it only works for schemas with no dependencies that can implement `Default`.

## [0.14.1]

### Added

- `nakago-axum`: Add a default Axum `AddLoaders` struct that wraps the base `nakago::config::AddLoaders` struct.
- `nakago-sea`: Add a default SeaOrm `AddLoaders` struct that wraps the base `nakago::config::AddLoaders` struct.

### Changed

- `nakago`: Introduced the `derive-new` crate and used it in a few places.
- `nakago-axum`: Introduced the `derive-new` crate and used it in a few places.

## [0.14.0]

### Changed

- `nakago`: Renamed `nakago::error` to `nakago::errors`
- `nakago-axum`: Moved away from a custom Axum State, using a hardcoded State that simply contains an `Inject` container instead.
- `nakago-axum`: Major routing improvements, with drastic simplification from the previous version.
- `nakago-async-graphql`: Reworked things around the new State approach for Axum.
- Updated examples.

## Added

- `nakago-axum`: Added `nakago-axum::State` and `nakago-axum::Inject` for smoother interop with Axum handlers.
- `nakago-async-graphql`: Added `nakago-async-graphql::errors::to_graphql_response` to convert `nakago::Error` into a GraphQL response.

## [0.13.0]

### Changed

- Updated imports, added new public `use` statements, and renamed types to conform to [RFC-356](https://github.com/rust-lang/rfcs/blob/master/text/0356-no-module-prefixes.md).

## [0.12.2]

### Changed

- Made `nakago-axum::auth::jwks::JWKSValidator` public so that it can be used in variations of the auth flow.

## [0.12.1]

### Added

- Added an optional Backtrace to the NotFound error, which uses the RUST_BACKTRACE variable to determine whether or not to include it.

### Changed

- Fixed the Config init process so that it would work even if you haven't added any loaders.

## [0.12.0]

### Changed

- Update Github Actions workflows
- Use a build tag for integration tests rather than ignoring them
- Simplify Docker Compose resource management for the example projects
- Added the config path to the TestUtils
- Added to the tutorial documentation

### Removed

- Simplified the DatabaseConfig

## [0.11.0]

### Added

- Added test utils to `nakago`, `nakago-axum`, and `nakago-async-graphql`.
- Added a convenience method to retrieve the config for an Application by tag or type.

### Changed

- Updated `nakago-axum` to make the mocked authenticate method injectable by DI rather than using build tags.
- Updated the Async-GraphQL example to streamline things and take advantage of the `TestUtils` provided in the library.

## [0.10.0]

### Added

- Added a Config tag and a State tag to the AxumApplication struct, to prevent the need for explicit turbofish syntax when injecting those dependencies.
- Added "modify" and "modify_type" convenience methods, making it easier to handle cases where you want to consume a dependency, modify it, and then immediately re-inject it.
- Added an `HttpConfigLoader` to the `nakago-axum` crate, and added it to the list of default config loaders for Axum.
- Added a `simple` example with a simple Axum HTTP application with authentication.
- Added to the `website/docs/tutorial.md` documentation.

## [0.9.0]

### Added

- Added the `nakago-async-graphql` library with an initial implementation of schema building.
- Added a new Lifecycle Event, `Load`, which is intended for use before the config is loaded. During this phase, an Application will typically set up all of its Providers and ConfigLoaders.
  - The `Init` Lifecycle Event is now used for constructing anything needed to run the app. This is typically where an Application initializes things like GraphQL schemas, Axum routes, or anything else that needs to make use of Provided dependencies or the loaded Config.
- Added the `Inject::override_tag()` and `Inject::override_type()` methods to allow for injecting a dependency whether or not it was already there, returning a boolean result to indicate whether the key already existed or not.

### Changed

- Re-organized the Async-GraphQL example to be more modular and easier to follow.
- Moved the Injector and Provider code into their own files within the `inject` module.

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

[unreleased]: https://github.com/bkonkle/nakago/compare/0.25.0...HEAD
[0.25.0]: https://github.com/bkonkle/nakago/compare/0.24.1...0.25.0
[0.24.1]: https://github.com/bkonkle/nakago/compare/0.24.0...0.24.1
[0.24.0]: https://github.com/bkonkle/nakago/compare/0.22.0...0.24.0
[0.23.0]: https://github.com/bkonkle/nakago/compare/0.22.0...0.23.0
[0.22.0]: https://github.com/bkonkle/nakago/compare/0.21.0...0.22.0
[0.21.0]: https://github.com/bkonkle/nakago/compare/0.20.0...0.21.0
[0.20.0]: https://github.com/bkonkle/nakago/compare/0.19.1...0.20.0
[0.19.1]: https://github.com/bkonkle/nakago/compare/0.19.0...0.19.1
[0.19.0]: https://github.com/bkonkle/nakago/compare/0.18.0...0.19.0
[0.18.0]: https://github.com/bkonkle/nakago/compare/0.17.0...0.18.0
[0.17.0]: https://github.com/bkonkle/nakago/compare/0.16.0...0.17.0
[0.16.0]: https://github.com/bkonkle/nakago/compare/0.15.0...0.16.0
[0.15.0]: https://github.com/bkonkle/nakago/compare/0.14.1...0.15.0
[0.14.1]: https://github.com/bkonkle/nakago/compare/0.14.0...0.14.1
[0.14.0]: https://github.com/bkonkle/nakago/compare/0.13.0...0.14.0
[0.13.0]: https://github.com/bkonkle/nakago/compare/0.12.2...0.13.0
[0.12.2]: https://github.com/bkonkle/nakago/compare/0.12.1...0.12.2
[0.12.1]: https://github.com/bkonkle/nakago/compare/0.12.0...0.12.1
[0.12.0]: https://github.com/bkonkle/nakago/compare/0.11.0...0.12.0
[0.11.0]: https://github.com/bkonkle/nakago/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/bkonkle/nakago/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/bkonkle/nakago/compare/0.8.0...0.9.0
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
