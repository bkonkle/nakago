use std::{any::Any, sync::Arc};

use crate::Dependency;

use super::{Inject, Key, Provider, Result};

impl Inject {
    /// Retrieve a reference to a Dependency if it exists. Return a NotFound error if the TypeId
    /// isn't present.
    pub async fn get<T: Any + Send + Sync>(&self) -> Result<Arc<T>> {
        self.get_key(Key::from_type_id::<T>()).await
    }

    /// Retrieve a reference to a Dependency if it exists.
    pub async fn get_opt<T: Any + Send + Sync>(&self) -> Result<Option<Arc<T>>> {
        self.get_key_opt(Key::from_type_id::<T>()).await
    }

    /// Override an existing Dependency directly, using core::future::ready to wrap it in an
    /// immediately resolving Pending Future. Return true if the Key was already present.
    pub async fn override_type<T: Any + Send + Sync>(&self, dep: T) -> Result<bool> {
        self.override_key(Key::from_type_id::<T>(), dep).await
    }

    /// Provide a Dependency directly, using core::future::ready to wrap it in an immediately
    /// resolving Pending Future.
    pub async fn inject<T: Any + Send + Sync>(&self, dep: T) -> Result<()> {
        self.inject_key(Key::from_type_id::<T>(), dep).await
    }

    /// Replace an existing Dependency directly, using core::future::ready to wrap it in an
    /// immediately resolving Pending Future. Return a NotFound error if the TypeId isn't present.
    pub async fn replace<T: Any + Send + Sync>(&self, dep: T) -> Result<()> {
        self.replace_key(Key::from_type_id::<T>(), dep).await
    }

    /// Inject a Dependency Provider
    pub async fn provide<T: Any + Send + Sync>(
        &self,
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Result<()> {
        self.provide_key::<T>(Key::from_type_id::<T>(), provider)
            .await
    }

    /// Inject a replacement Dependency Provider if the TypeId is present
    pub async fn replace_with<T: Any + Send + Sync>(
        &self,
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Result<()> {
        self.replace_key_with::<T>(Key::from_type_id::<T>(), provider)
            .await
    }

    /// Remove a Dependency from the container and try to unwrap it from the Arc, which will only
    /// succeed if there are no other strong pointers to the value. Any Arcs handed out will still
    /// be valid, but the container will no longer hold a reference. Return a NotFound error if the
    /// TypeId isn't present.
    pub async fn consume<T: Any + Send + Sync>(&self) -> Result<T> {
        self.consume_key(Key::from_type_id::<T>()).await
    }

    /// Remove a Dependency from the container and try to unwrap it from the Arc, which will only
    /// succeed if there are no other strong pointers to the value. Any Arcs handed out will still
    /// be valid, but the container will no longer hold a reference.
    pub async fn consume_opt<T: Any + Send + Sync>(&self) -> Result<Option<T>> {
        self.consume_key_opt(Key::from_type_id::<T>()).await
    }

    /// Temporarily remove a dependency from the container and try to unwrap it from the Arc, which
    /// will only succeed if there are no other strong pointers to the value. Then, apply a function
    /// to it, and then injects it back into the container.
    pub async fn modify<T, F>(&self, modify: F) -> Result<()>
    where
        T: Any + Send + Sync,
        F: FnOnce(T) -> Result<T>,
    {
        self.modify_key(Key::from_type_id::<T>(), modify).await
    }

    /// Discard a Dependency from the container. Any Arcs handed out will still be valid, but
    /// the container will no longer hold a reference.
    pub async fn remove<T: Any + Send + Sync>(&self) -> Result<()> {
        self.remove_key(Key::from_type_id::<T>()).await
    }

    /// Destroy the container and discard all Dependencies except for the given TypeId. Any Arcs
    /// handed out will still be valid, but the container will be fully unloaded and all references
    /// will be dropped. Return a NotFound error if the TypeId isn't present.
    pub async fn eject<T: Any + Send + Sync>(self) -> Result<T> {
        self.eject_key(Key::from_type_id::<T>()).await
    }

    /// Destroy the container and discard all Dependencies except for the given TypeId. Any Arcs
    /// handed out will still be valid, but the container will be fully unloaded and all references
    /// will be dropped.
    pub async fn eject_opt<T: Any + Send + Sync>(self) -> Result<Option<T>> {
        self.eject_key_opt(Key::from_type_id::<T>()).await
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::any::type_name;

    use fake::Fake;
    use googletest::{assert_that, prelude::starts_with};

    use crate::{
        container::test::{HasId, OtherService, TestService},
        errors::Result,
        provider::test::{HasIdProvider, OtherServiceProvider, TestServiceProvider},
    };

    use super::*;

    #[tokio::test]
    async fn test_inject_success() -> Result<()> {
        let i = Inject::default();

        let service = TestService::new(fake::uuid::UUIDv4.fake());

        i.inject(service).await?;

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

        i.inject(TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Inject the same type a second time
        let result = i.inject(TestService::new(fake::uuid::UUIDv4.fake())).await;

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

        i.inject(TestService::new(expected.clone())).await?;

        let result = i.get_opt::<TestService>().await?.unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_opt_vec_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(vec![TestService::new(expected.clone())]).await?;

        let result = i.get_opt::<Vec<TestService>>().await?.unwrap();

        assert_eq!(expected, result[0].id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_opt_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_opt::<TestService>().await?;

        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(TestService::new(expected.clone())).await?;

        let result = i.get::<TestService>().await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_dyn_get_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject::<Box<dyn HasId>>(Box::new(TestService::new(expected.clone())))
            .await?;

        let repo = i.get::<Box<dyn HasId>>().await?;

        assert_eq!(expected, repo.get_id());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get::<TestService>().await;

        if let Err(err) = result {
            let expected = format!(
                "{} was not found\n\nAvailable: (empty)",
                type_name::<TestService>(),
            );

            assert!(err.to_string().contains(&expected));
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override the instance that was injected the first time
        i.replace(TestService {
            id: expected.clone(),
        })
        .await?;

        let result = i.get::<TestService>().await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_not_found() -> Result<()> {
        let i = Inject::default();

        i.inject(Box::new(TestService::new(fake::uuid::UUIDv4.fake())))
            .await?;
        i.inject::<Box<dyn HasId>>(Box::new(OtherService::new(fake::uuid::UUIDv4.fake())))
            .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace(Box::new(OtherService {
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

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        let service = i.get::<TestService>().await?;

        assert_eq!(service.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_type_multiple_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.provide::<OtherService>(OtherServiceProvider::new(expected_other.clone()))
            .await?;

        let service = i.get::<TestService>().await?;
        let other = i.get::<OtherService>().await?;

        assert_eq!(service.id, expected);
        assert_eq!(other.other_id, expected_other);

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_type_dyn_success() -> Result<()> {
        let i = Inject::default();

        i.provide::<Box<dyn HasId>>(HasIdProvider::default())
            .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_type_id::<Box<dyn HasId>>()),
            "key does not exist in injection container"
        );

        let service = i.get::<Box<dyn HasId>>().await?;

        assert_eq!(service.get_id(), "test-service");

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_type_occupied() -> Result<()> {
        let i = Inject::default();

        let expected = format!("{} has already been provided", type_name::<TestService>());

        i.provide::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        let result = i
            .provide::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
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

        i.provide::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Retrieve the dependency once to trigger the Provider
        i.get::<TestService>().await?;

        // Override the instance that was injected the first time
        i.replace_with::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.get::<TestService>().await?;

        assert_eq!(result.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_type_with_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        i.provide::<OtherService>(OtherServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Retrieve the dependencies once to trigger the Providers
        i.get::<TestService>().await?;
        i.get::<OtherService>().await?;

        // Override the instances that were injected the first time
        i.replace_with::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.replace_with::<OtherService>(OtherServiceProvider::new(expected_other.clone()))
            .await?;

        let result = i.get::<TestService>().await?;
        let other = i.get::<OtherService>().await?;

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

        i.provide::<TestService>(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_with::<OtherService>(OtherServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await;

        if let Err(err) = result {
            assert!(err.to_string().contains(&expected));
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_type_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.consume::<TestService>().await?;

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

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        // Trigger the Provider so that the value is Pending below
        i.get::<TestService>().await?;

        let result = i.consume::<TestService>().await?;

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
    async fn test_consume_type_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i
            .consume::<TestService>()
            .await
            .expect_err("Did not error as expected");

        assert!(result
            .to_string()
            .starts_with("nakago::container::test::TestService"));

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_type_provider_in_use() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let _borrow = i.get::<TestService>().await?;

        let result = i.consume::<TestService>().await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_type_pending_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.provide::<OtherService>(OtherServiceProvider::new(expected_other.clone()))
            .await?;

        // Trigger the Providers so that the values are Pending below
        i.get::<TestService>().await?;
        i.get::<OtherService>().await?;

        let result = i.consume::<TestService>().await?;
        let other = i.consume::<OtherService>().await?;

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

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        i.remove::<TestService>().await?;

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

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        // Trigger the Provider so that the value is Pending below
        i.get::<TestService>().await?;

        i.remove::<TestService>().await?;

        assert!(
            !i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_type_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i
            .remove::<TestService>()
            .await
            .expect_err("Did not error as expected");

        assert!(result
            .to_string()
            .starts_with("nakago::container::test::TestService"));

        Ok(())
    }

    #[tokio::test]
    async fn test_modify_type_success() -> Result<()> {
        let i = Inject::default();

        let initial: String = fake::uuid::UUIDv4.fake();
        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(initial.clone()))
            .await?;

        i.modify::<TestService, _>(|mut t| {
            t.id.clone_from(&expected);

            Ok(t)
        })
        .await?;

        let result = i.get::<TestService>().await?;

        assert_eq!(result.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_modify_type_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i
            .modify::<TestService, _>(|mut t| {
                t.id = "test".to_string();

                Ok(t)
            })
            .await
            .expect_err("Did not error as expected");

        assert_that!(
            result.to_string(),
            starts_with("nakago::container::test::TestService")
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_modify_type_in_use() -> Result<()> {
        let i = Inject::default();

        let initial: String = fake::uuid::UUIDv4.fake();
        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(initial.clone()))
            .await?;

        let _borrow = i.get::<TestService>().await?;

        let result = i
            .modify::<TestService, _>(|mut t| {
                t.id.clone_from(&expected);

                Ok(t)
            })
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_eject_key_pending_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject::<TestService>(TestService::new(expected.clone()))
            .await?;

        let service = i.eject::<TestService>().await?;

        // This is commented out because it demonstrates a case prevented by the compiler - usage
        // of the container after ejection.
        //
        // i.get::<TestService>().await?;

        assert_eq!(service.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_eject_key_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let service = i.eject::<TestService>().await?;

        assert_eq!(service.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_eject_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i
            .eject::<TestService>()
            .await
            .expect_err("Did not error as expected");

        assert!(result
            .to_string()
            .starts_with("nakago::container::test::TestService"));

        Ok(())
    }

    #[tokio::test]
    async fn test_eject_key_provider_in_use() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide::<TestService>(TestServiceProvider::new(expected.clone()))
            .await?;

        let _borrow = i.get::<TestService>().await?;

        let result = i.eject::<TestService>().await;

        assert!(result.is_err());

        Ok(())
    }
}
