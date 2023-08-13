use std::{any::Any, sync::Arc};

use super::{container::Dependency, Inject, Key, Provider, Result};

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
        provider: Arc<dyn Provider<Dependency>>,
    ) -> Result<()> {
        self.provide_key(Key::from_type_id::<T>(), provider).await
    }

    /// Replace an existing Provider for a type-id dependency
    pub async fn replace_type_with<T: Any + Send + Sync>(
        &self,
        provider: Arc<dyn Provider<Dependency>>,
    ) -> Result<()> {
        self.replace_key_with(Key::from_type_id::<T>(), provider)
            .await
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::any::type_name;

    use crate::inject::container::test::{HasId, OtherService, TestService};

    use super::*;

    #[tokio::test]
    async fn test_inject_success() -> Result<()> {
        let mut i = Inject::default();

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
        let mut i = Inject::default();

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
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone())).await?;

        let result = i.get_type_opt::<TestService>().await?.unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_opt_vec_success() -> Result<()> {
        let mut i = Inject::default();

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
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_type(TestService::new(expected.clone())).await?;

        let result = i.get_type::<TestService>().await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_dyn_get_success() -> Result<()> {
        let mut i = Inject::default();

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
        let mut i = Inject::default();

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
        let mut i = Inject::default();

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
}
