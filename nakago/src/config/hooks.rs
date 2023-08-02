use std::sync::Arc;

use crate::{Inject, Pending};

use super::{Loader, CONFIG_LOADERS};

/// Add the given Config Loaders to the stack. Injects `Tag(ConfigLoaders)` if it has not been
/// provided yet.
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
pub async fn add_loaders(
    loaders: Vec<Arc<dyn Loader>>,
) -> impl for<'a> FnOnce(&'a mut Inject<'a>) -> Pending<'a> {
    |i| {
        Box::pin(async move {
            let mut loaders = match i.consume(&CONFIG_LOADERS).await {
                Ok(existing) => existing,
                Err(_) => Vec::new(),
            };

            // Add the given ConfigLoaders to the stack
            for loader in loaders.clone() {
                loaders.push(loader);
            }

            i.inject(&CONFIG_LOADERS, loaders)?;

            Ok(())
        })
    }
}
