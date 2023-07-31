use std::{any::Any, collections::HashMap, fmt::Debug};

use super::{Error, Key, Result};

/// A type map for dependency injection
pub(crate) type TypeMap = HashMap<Key, Box<dyn Any + Send + Sync>>;

/// The injection Container
#[derive(Default, Debug)]
pub struct Inject(pub(crate) TypeMap);

// The base methods powering both the Tag and TypeId modes
impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<&T> {
        self.get_key_opt::<T>(key.clone())?
            .ok_or_else(|| Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            })
    }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    pub(crate) fn get_key_mut<T: Any + Send + Sync>(&mut self, key: Key) -> Result<&mut T> {
        let available = self.available_type_names();

        self.get_key_mut_opt::<T>(key.clone())?
            .ok_or(Error::NotFound {
                missing: key,
                available,
            })
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub(crate) fn get_key_opt<T: Any + Send + Sync>(&self, key: Key) -> Result<Option<&T>> {
        if let Some(d) = self.0.get(&key) {
            if let Some(dep) = d.downcast_ref::<T>() {
                Ok(Some(dep))
            } else {
                Err(Error::TypeMismatch {
                    key,
                    type_name: std::any::type_name::<T>().to_string(),
                })
            }
        } else {
            Ok(None)
        }
    }

    /// Retrieve a mutable reference to a dependency if it exists in the map
    pub(crate) fn get_key_mut_opt<T: Any + Send + Sync>(
        &mut self,
        key: Key,
    ) -> Result<Option<&mut T>> {
        if let Some(d) = self.0.get_mut(&key) {
            if let Some(dep) = d.downcast_mut::<T>() {
                Ok(Some(dep))
            } else {
                Err(Error::TypeMismatch {
                    key,
                    type_name: std::any::type_name::<T>().to_string(),
                })
            }
        } else {
            Ok(None)
        }
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if self.0.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let _ = self.0.insert(key, Box::new(dep));

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

        self.0.insert(key, Box::new(dep));

        Ok(())
    }

    /// Remove a dependency from the map and return it for use
    pub(crate) fn consume_key<T: Any + Send + Sync>(&mut self, key: Key) -> Result<T> {
        self.0
            .remove(&key)
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
