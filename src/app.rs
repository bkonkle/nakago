use std::{
    any::Any,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use crate::{
    config::{self, Config},
    inject,
};

/// State must be clonable and able to be stored in the Inject container
pub trait State: Clone + Any + Send + Sync {}

/// The top-level Application struct
#[derive(Default)]
pub struct Application<C: Config> {
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
    /// Initialize the App
    ///
    /// **Provides:**
    ///   - `C: Config`
    pub async fn init(&mut self, config_path: Option<PathBuf>) -> inject::Result<()> {
        // Initialize the Config using the given path
        config::init::<C>(&mut self.i, config_path).await?;

        Ok(())
    }
}
