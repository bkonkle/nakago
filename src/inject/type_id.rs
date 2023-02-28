use std::any::Any;

use super::{Key, Result};
use crate::Inject;

impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub fn get_type<T: Any + Send + Sync>(&self) -> Result<&T> {
        self.get_key(Key::from_type_id::<T>())
    }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    pub fn get_type_mut<T: Any + Send + Sync>(&mut self) -> Result<&mut T> {
        self.get_key_mut(Key::from_type_id::<T>())
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub fn get_type_opt<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.get_key_opt(Key::from_type_id::<T>())
    }

    /// Retrieve a mutable reference to a dependency if it exists in the map
    pub fn get_type_mut_opt<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.get_key_mut_opt(Key::from_type_id::<T>())
    }

    /// Provide a dependency directly
    pub fn inject_type<T: Any + Send + Sync>(&mut self, dep: T) -> Result<()> {
        self.inject_key(Key::from_type_id::<T>(), dep)
    }

    /// Replace an existing dependency directly
    pub fn replace_type<T: Any + Send + Sync>(&mut self, dep: T) -> Result<()> {
        self.replace_key(Key::from_type_id::<T>(), dep)
    }

    /// Consume a dependency, removing it from the container and moving it to the caller
    pub fn consume_type<T: Any + Send + Sync>(&mut self) -> Result<T> {
        self.consume_key(Key::from_type_id::<T>())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::any::type_name;

    use crate::inject::container::test::{HasId, OtherService, TestService};

    use super::*;

    #[test]
    fn test_inject_success() -> Result<()> {
        let mut i = Inject::default();

        let service = TestService::new(fake::uuid::UUIDv4.fake());

        i.inject_type(service)?;

        assert!(
            i.0.contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[test]
    fn test_inject_occupied() -> Result<()> {
        let mut i = Inject::default();

        i.inject_type(TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Inject the same type a second time
        let result = i.inject_type(TestService::new(fake::uuid::UUIDv4.fake()));

        if let Err(err) = result {
            assert_eq!(
                format!("{} has already been provided", type_name::<TestService>()),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[test]
    fn test_get_opt_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone()))?;

        let result = i.get_type_opt::<TestService>().unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_opt_vec_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(vec![TestService::new(expected.clone())])?;

        let result = i.get_type_opt::<Vec<TestService>>().unwrap();

        assert_eq!(expected, result[0].id);

        Ok(())
    }

    #[test]
    fn test_get_opt_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_type_opt::<TestService>();

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_get_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone()))?;

        let result = i.get_type::<TestService>()?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_dyn_get_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type::<Box<dyn HasId>>(Box::new(TestService::new(expected.clone())))?;

        let repo = i.get_type::<Box<dyn HasId>>()?;

        assert_eq!(expected, repo.get_id());

        Ok(())
    }

    #[test]
    fn test_get_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_type::<TestService>();

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable: (empty)",
                    type_name::<TestService>(),
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[test]
    fn test_get_mut_opt_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone()))?;

        let result = i.get_type_mut_opt::<TestService>().unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_mut_opt_not_found() -> Result<()> {
        let mut i = Inject::default();

        let result = i.get_type_mut_opt::<TestService>();

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_get_mut_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(vec![TestService::new(fake::uuid::UUIDv4.fake())])?;

        let services = i.get_type_mut::<Vec<TestService>>()?;
        services.push(TestService::new(expected.clone()));

        let result = i.get_type::<Vec<TestService>>()?;

        assert_eq!(expected, result[1].id);

        Ok(())
    }

    #[test]
    fn test_get_mut_not_found() -> Result<()> {
        let mut i = Inject::default();

        let result = i.get_type_mut::<TestService>();

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable: (empty)",
                    type_name::<TestService>(),
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[test]
    fn test_replace_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Override the instance that was injected the first time
        i.replace_type(TestService {
            id: expected.clone(),
        })?;

        let result = i.get_type::<TestService>()?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_replace_not_found() -> Result<()> {
        let mut i = Inject::default();

        i.inject_type(Box::new(TestService::new(fake::uuid::UUIDv4.fake())))?;
        i.inject_type::<Box<dyn HasId>>(Box::new(OtherService::new(fake::uuid::UUIDv4.fake())))?;

        // Override a type that doesn't have any instances yet
        let result = i.replace_type(Box::new(OtherService {
            other_id: fake::uuid::UUIDv4.fake(),
        }));

        if let Err(err) = result {
            let message = err.to_string();

            assert!(message.contains(&format!(
                "{} was not found\n\nAvailable:",
                type_name::<Box<OtherService>>()
            )));

            assert!(message.contains(&format!("\n - {}", type_name::<Box<TestService>>())));
            assert!(message.contains(&format!("\n - {}", type_name::<Box<dyn HasId>>())));
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }
}
