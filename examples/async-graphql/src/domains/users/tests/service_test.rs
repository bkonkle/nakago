use anyhow::Result;
use fake::{Fake, Faker};
use nakago::{inject, Inject};
use nakago_sea_orm::connection;
use pretty_assertions::assert_eq;
use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockExecResult, Transaction};

use crate::domains::users::{
    model::User,
    mutation::UpdateUserInput,
    service::{self, Service},
};

async fn setup(db: MockDatabase) -> inject::Result<Inject> {
    let i = Inject::default();

    i.provide::<DatabaseConnection>(connection::ProvideMock::new(db))
        .await?;

    i.provide::<Box<dyn Service>>(service::Provide::default())
        .await?;

    Ok(i)
}

#[tokio::test]
async fn test_users_service_get() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results(vec![vec![user.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service.get(&user.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, Some(user.clone()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
            vec![user.id.into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_users_service_get_by_username() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results(vec![vec![user.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service.get_by_username(&user.username, &false).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, Some(user));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."username" = $1 LIMIT $2"#,
            vec!["test-username".into(), 1u64.into()]
        )]
    );

    Ok(())
}

// TODO: test_users_service_get_by_username_with_roles

#[tokio::test]
async fn test_users_service_create() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results(vec![vec![user.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service.create(&user.username).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, user);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "users" ("username") VALUES ($1) RETURNING "id", "created_at", "updated_at", "username", "is_active""#,
            vec![user.username.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_users_service_update() -> Result<()> {
    let mut user: User = Faker.fake();
    user.username = "test-username".to_string();

    let updated = User {
        username: "updated-username".to_string(),
        roles: vec![],
        ..user.clone()
    };

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()], vec![updated.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .update(
            &user.id,
            &UpdateUserInput {
                username: Some(updated.username.clone()),
                is_active: None,
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, updated.clone());

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "users" SET "username" = $1 WHERE "users"."id" = $2 RETURNING "id", "created_at", "updated_at", "username", "is_active""#,
                vec![updated.username.into(), user.id.into()]
            )
        ]
    );

    Ok(())
}

// TODO: test_users_service_update_with_roles

#[tokio::test]
async fn test_users_service_delete() -> Result<()> {
    let mut user: User = Faker.fake();
    user.username = "test-username".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![user.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    service.delete(&user.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "users"."id", "users"."created_at", "users"."updated_at", "users"."username", "users"."is_active" FROM "users" WHERE "users"."id" = $1 LIMIT $2"#,
                vec![user.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "users" WHERE "users"."id" = $1"#,
                vec![user.id.into()]
            )
        ]
    );

    Ok(())
}
