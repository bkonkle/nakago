use std::{any::Any, collections::HashMap, future::Future, pin::Pin};

use super::{Error, Key, Result};

/// A type map for dependency injection
pub(crate) type TypeMap = HashMap<Key, Injector<dyn Any + Send + Sync>>;

pub struct Injector<T>
where
    T: Any + Send + Sync + ?Sized,
{
    instance: Option<Box<T>>,
    provider: Option<Box<dyn FnOnce(&Inject) -> Pin<Box<dyn Future<Output = Result<Box<T>>>>>>>,
}

/// The injection Container
#[derive(Default)]
pub struct Inject(pub(crate) TypeMap);

// The base methods powering both the Tag and TypeId modes
impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) async fn get_key<T: Any + Send + Sync>(&mut self, key: Key) -> Result<&T> {
        let available = self.available_type_names();

        if let Some(injector) = self.0.get_mut(&key) {
            if let Some(instance) = &injector.instance {
                return Ok(instance
                    .downcast_ref::<T>()
                    .ok_or_else(|| Error::TypeMismatch(key))?);
            }

            if let Some(provider) = injector.provider.take() {
                let instance = provider(self).await?;

                injector.instance = Some(instance);

                return injector
                    .instance
                    .as_ref()
                    .ok_or_else(|| Error::NotFound {
                        missing: key.clone(),
                        available,
                    })
                    .and_then(|instance| {
                        instance
                            .downcast_ref::<T>()
                            .ok_or_else(|| Error::TypeMismatch(key))
                    });
            }
        }

        Err(Error::NotFound {
            missing: key.clone(),
            available,
        })
    }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    pub(crate) fn get_key_mut<T: Any + Send + Sync>(&mut self, key: Key) -> Result<&mut T> {
        if let Some(injector) = self.0.get_mut(&key) {
            if let Some(instance) = &mut injector.instance {
                return instance
                    .downcast_mut::<T>()
                    .ok_or_else(|| Error::TypeMismatch(key));
            }
        };

        Err(Error::NotFound {
            missing: key.clone(),
            available: self.available_type_names(),
        })
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if self.0.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let _ = self.0.insert(
            key,
            Injector {
                instance: Some(Box::new(dep)),
                provider: None,
            },
        );

        Ok(())
    }

    /// Replace an existing dependency directly
    pub(crate) fn replace_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if !self.0.contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            });
        }

        self.0.insert(
            key,
            Injector {
                instance: Some(Box::new(dep)),
                provider: None,
            },
        );

        Ok(())
    }

    /// Remove a dependency from the map and return it for use
    pub(crate) fn consume_key<T: Any + Send + Sync>(&mut self, key: Key) -> Result<T> {
        self.0
            .remove(&key)
            .and_then(|opt| opt.instance)
            .ok_or_else(|| Error::NotFound {
                missing: key.clone(),
                available: self.available_type_names(),
            })
            .and_then(|d| d.downcast().map_err(|_err| Error::CannotConsume(key)))
            .map(|d| *d)
    }

    /// Return a list of all available type names in the map
    pub(crate) fn available_type_names(&self) -> Vec<Key> {
        self.0.keys().cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    pub trait HasId: Send + Sync {
        fn get_id(&self) -> String;
    }

    pub struct TestService {
        pub(crate) id: String,
    }

    impl TestService {
        pub fn new(id: String) -> Self {
            Self { id }
        }
    }

    impl HasId for TestService {
        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    pub struct OtherService {
        pub(crate) other_id: String,
    }

    impl OtherService {
        pub fn new(other_id: String) -> Self {
            Self { other_id }
        }
    }

    impl HasId for OtherService {
        fn get_id(&self) -> String {
            self.other_id.clone()
        }
    }
}
