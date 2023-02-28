# Applications

ℹ️ - This documentation is a work in progress. Stay tuned for more details!

## System

To manage the lifecycle of an application, the top-level `nakago::System` struct provides Init and Startup hooks. More hooks, like a Shutdown hook, are coming soon.

### Lifecycle Hooks

**Init**

The `init` Hook is invoked before the Config Loaders tag is consumed and the loaders are used to initialize the Config.

**Startup**

The `startup` Hook is invoked after the Config is loaded, but before the Application is run.

## HTTP Applications

The `nakago-axum` crate defines `HttpApplication`, which wraps `System` and provides a way to Run an HTTP application.

### HTTP Application Lifecycle

**Init**

The `init` Hook for an HTTP Application automatically adds the `HttpConfig` and `AuthConfig` Config Loaders before the user-provided hook is invoked and the Config is generated.

**Startup**

The `startup` Hook for an HTTP Application uses the State with the provided Router, allowing the flow of the application to proceed through the Axum request handlers.
