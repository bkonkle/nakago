use std::{
    any::Any,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use crate::{
    config::{loader::Config, providers::ConfigInitializer},
    inject,
};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// The top-level Application struct
pub struct Application<C: Config> {
    initializers: Vec<Box<dyn inject::Initializer>>,
    i: inject::Inject,
    _phantom: PhantomData<C>,
}

impl<C: Config> Deref for Application<C> {
    type Target = inject::Inject;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl<C: Config> DerefMut for Application<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.i
    }
}

impl<C: Config + Debug> Application<C> {
    /// Create a new Application instance
    pub fn new(initializers: Vec<Box<dyn inject::Initializer>>) -> Self {
        Self {
            initializers,
            i: inject::Inject::default(),
            _phantom: Default::default(),
        }
    }

    /// Create the Inject container and run initializers
    ///
    /// **Depends on:**
    ///   - `C: Config`
    pub async fn initialize(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // 先ず First of all, initialize the Config
        self.i
            .init(if let Some(config_path) = config_path {
                vec![Box::new(ConfigInitializer::<C>::with_custom_path(
                    config_path,
                ))]
            } else {
                vec![Box::new(ConfigInitializer::<C>::default())]
            })
            .await?;

        for initializer in &self.initializers {
            initializer.init(&mut self.i).await?;
        }

        Ok(())
    }
}
