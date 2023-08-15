use std::{any::Any, sync::Arc};

use super::{Inject, Key, Provider, Result};

impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub async fn get_type<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.get_key(Key::from_type_id::<T>()).await
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub async fn get_type_opt<T: Any + Send + Sync>(&self) -> Result<Option<Arc<T>>> {
        self.get_key_opt(Key::from_type_id::<T>()).await
    }

    /// Provide a dependency directly
    pub async fn inject_type<T: Any + Send + Sync>(&self, dep: T) -> Result<()> {
        self.inject_key(Key::from_type_id::<T>(), dep).await
    }

    /// Replace an existing dependency directly
    pub async fn replace_type<T: Any + Send + Sync>(&self, dep: T) -> Result<()> {
        self.replace_key(Key::from_type_id::<T>(), dep).await
    }

    /// Register a Provider for a type-id dependency
    pub async fn provide_type<T: Any + Send + Sync>(
        &self,
        provider: impl Provider + 'static,
    ) -> Result<()> {
        self.provide_key(Key::from_type_id::<T>(), provider).await
    }

    /// Replace an existing Provider for a type-id dependency
    pub async fn replace_type_with<T: Any + Send + Sync>(
        &self,
        provider: impl Provider + 'static,
    ) -> Result<()> {
        self.replace_key_with(Key::from_type_id::<T>(), provider)
            .await
    }

    /// Consume a tagged dependency, removing it from the container, returning an error if not found
    pub async fn consume_type<T: Any + Send + Sync>(&self) -> Result<T> {
        self.consume_key(Key::from_type_id::<T>()).await
    }

    /// Consume a tagged dependency, removing it from the container
    pub async fn consume_type_opt<T: Any + Send + Sync>(&self) -> Result<Option<T>> {
        self.consume_key_opt(Key::from_type_id::<T>()).await
    }

    /// Remove a tagged dependency from the container, returning an error if not found
    pub async fn remove_type<T: Any + Send + Sync>(&self) -> Result<()> {
        self.remove_key(Key::from_type_id::<T>()).await
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::any::type_name;

    use crate::inject::container::test::{
        HasId, HasIdProvider, OtherService, OtherServiceProvider, TestService, TestServiceProvider,
    };

    use super::*;

    #[tokio::test]
    async fn test_inject_success() -> Result<()> {
        let i = Inject::default();

        let service = TestService::new(fake::uuid::UUIDv4.fake());

        i.inject_type(service).await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_inject_occupied() -> Result<()> {
        let i = Inject::default();

        i.inject_type(TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Inject the same type a second time
        let result = i
            .inject_type(TestService::new(fake::uuid::UUIDv4.fake()))
            .await;

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

    #[tokio::test]
    async fn test_get_opt_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone())).await?;

        let result = i.get_type_opt::<TestService>().await?.unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_opt_vec_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(vec![TestService::new(expected.clone())])
            .await?;

        let result = i.get_type_opt::<Vec<TestService>>().await?.unwrap();

        assert_eq!(expected, result[0].id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_opt_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_type_opt::<TestService>().await?;

        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone())).await?;

        let result = i.get_type::<TestService>().await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_dyn_get_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type::<Box<dyn HasId>>(Box::new(TestService::new(expected.clone())))
            .await?;

        let repo = i.get_type::<Box<dyn HasId>>().await?;

        assert_eq!(expected, repo.get_id());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_type::<TestService>().await;

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

    #[tokio::test]
    async fn test_replace_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override the instance that was injected the first time
        i.replace_type(TestService {
            id: expected.clone(),
        })
        .await?;

        let result = i.get_type::<TestService>().await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_not_found() -> Result<()> {
        let i = Inject::default();

        i.inject_type(Box::new(TestService::new(fake::uuid::UUIDv4.fake())))
            .await?;
        i.inject_type::<Box<dyn HasId>>(Box::new(OtherService::new(fake::uuid::UUIDv4.fake())))
            .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_type(Box::new(OtherService {
                other_id: fake::uuid::UUIDv4.fake(),
            }))
            .await;

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

    #[tokio::test]
    async fn test_provide_type_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        let service = i.get_type::<TestService>().await?;

        assert_eq!(service.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_type_multiple_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.provide_type::<OtherService>(OtherServiceProvider::new(expected_other.clone()))
            .await?;

        let service = i.get_type::<TestService>().await?;
        let other = i.get_type::<OtherService>().await?;

        assert_eq!(service.id, expected);
        assert_eq!(other.other_id, expected_other);

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_type_dyn_success() -> Result<()> {
        let i = Inject::default();

        i.provide_type::<Box<dyn HasId>>(HasIdProvider::default())
            .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_type_id::<Box<dyn HasId>>()),
            "key does not exist in injection container"
        );

        let service = i.get_type::<Box<dyn HasId>>().await?;

        assert_eq!(service.get_id(), "test-service");

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_type_occupied() -> Result<()> {
        let i = Inject::default();

        let expected = format!("{} has already been provided", type_name::<TestService>());

        i.provide_type::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        let result = i
            .provide_type::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await;

        if let Err(err) = result {
            assert_eq!(err.to_string(), expected);
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_type_with_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Retrieve the dependency once to trigger the Provider
        i.get_type::<TestService>().await?;

        // Override the instance that was injected the first time
        i.replace_type_with::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.get_type::<TestService>().await?;

        assert_eq!(result.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_type_with_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        i.provide_type::<OtherService>(OtherServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Retrieve the dependencies once to trigger the Providers
        i.get_type::<TestService>().await?;
        i.get_type::<OtherService>().await?;

        // Override the instances that were injected the first time
        i.replace_type_with::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.replace_type_with::<OtherService>(OtherServiceProvider::new(expected_other.clone()))
            .await?;

        let result = i.get_type::<TestService>().await?;
        let other = i.get_type::<OtherService>().await?;

        assert_eq!(result.id, expected);
        assert_eq!(other.other_id, expected_other);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_type_with_not_found() -> Result<()> {
        let i = Inject::default();

        let expected = format!(
            "{} was not found\n\nAvailable:\n - {}",
            type_name::<OtherService>(),
            type_name::<TestService>()
        );

        i.provide_type::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_type_with::<OtherService>(OtherServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await;

        if let Err(err) = result {
            assert_eq!(err.to_string(), expected);
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_type_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.consume_type::<TestService>().await?;

        assert_eq!(result.id, expected);

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_type_pending_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        // Trigger the Provider so that the value is Pending below
        i.get_type::<TestService>().await?;

        let result = i.consume_type::<TestService>().await?;

        assert_eq!(result.id, expected);

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_type_pending_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.provide_type::<OtherService>(OtherServiceProvider::new(expected_other.clone()))
            .await?;

        // Trigger the Providers so that the values are Pending below
        i.get_type::<TestService>().await?;
        i.get_type::<OtherService>().await?;

        let result = i.consume_type::<TestService>().await?;
        let other = i.consume_type::<OtherService>().await?;

        assert_eq!(result.id, expected);
        assert_eq!(other.other_id, expected_other);

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key still exists in injection container"
        );

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<OtherService>()),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_type_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.remove_type::<TestService>().await?;

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_type_pending_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_type::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        // Trigger the Provider so that the value is Pending below
        i.get_type::<TestService>().await?;

        i.remove_type::<TestService>().await?;

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key still exists in injection container"
        );

        Ok(())
    }
}
