use std::{any::Any, fmt::Debug};

use serde::{Deserialize, Serialize};

/// Config is the final loaded result
pub trait Config:
    Any + Clone + Debug + Default + Serialize + Send + Sync + for<'a> Deserialize<'a>
{
}
