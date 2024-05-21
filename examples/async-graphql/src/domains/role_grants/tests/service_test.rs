use anyhow::Result;
use fake::{Fake, Faker};
use nakago::{inject, Inject};
use nakago_sea_orm::{connection, CONNECTION};
use pretty_assertions::assert_eq;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction};

use crate::domains::{
    role_grants::{
        model::{CreateRoleGrantInput, RoleGrant},
        service::{self, SERVICE},
    },
    users::model::User,
};

async fn setup(db: MockDatabase) -> inject::Result<Inject> {
    let i = Inject::default();

    i.provide(&CONNECTION, connection::ProvideMock::new(db))
        .await?;

    i.provide(&SERVICE, service::Provide::default()).await?;

    Ok(i)
}

#[tokio::test]
async fn test_role_grants_service_get() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut grant: RoleGrant = Faker.fake();
    grant.id = format!("{}-{}", user.id, "profile-id");
    grant.user_id.clone_from(&user.id);
    grant.resource_table = "profiles".to_string();
    grant.resource_id = "profile-id".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![grant.clone()]]),
    )
    .await?;

    let service = i.get(&SERVICE).await?;

    let result = service.get(&grant.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject(&CONNECTION).await?;

    assert_eq!(result, Some(grant));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "role_grants"."id", "role_grants"."created_at", "role_grants"."updated_at", "role_grants"."role_key", "role_grants"."user_id", "role_grants"."resource_table", "role_grants"."resource_id" FROM "role_grants" WHERE "role_grants"."id" = $1 LIMIT $2"#,
            vec![format!("{}-{}", user.id, "profile-id").into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_role_grants_service_create() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut grant: RoleGrant = Faker.fake();
    grant.id = format!("{}-{}", user.id, "profile-id");
    grant.user_id.clone_from(&user.id);
    grant.resource_table = "profiles".to_string();
    grant.resource_id = "profile-id".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![grant.clone()]]),
    )
    .await?;

    let service = i.get(&SERVICE).await?;

    let result = service
        .create(&CreateRoleGrantInput {
            role_key: grant.role_key.clone(),
            user_id: user.id.clone(),
            resource_table: "profiles".to_string(),
            resource_id: "profile-id".to_string(),
        })
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject(&CONNECTION).await?;

    assert_eq!(result, grant);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "role_grants" ("role_key", "user_id", "resource_table", "resource_id") VALUES ($1, $2, $3, $4) RETURNING "id", "created_at", "updated_at", "role_key", "user_id", "resource_table", "resource_id""#,
            vec![
                grant.role_key.into(),
                grant.user_id.into(),
                grant.resource_table.into(),
                grant.resource_id.into()
            ]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_role_grants_service_delete() -> Result<()> {
    let mut user: User = Faker.fake();
    user.roles = vec![];
    user.username = "test-username".to_string();

    let mut grant: RoleGrant = Faker.fake();
    grant.id = format!("{}-{}", user.id, "profile-id");
    grant.user_id = user.id;
    grant.resource_table = "profiles".to_string();
    grant.resource_id = "profile-id".to_string();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![grant.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }]),
    )
    .await?;

    let service = i.get(&SERVICE).await?;

    service.delete(&grant.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject(&CONNECTION).await?;

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "role_grants"."id", "role_grants"."created_at", "role_grants"."updated_at", "role_grants"."role_key", "role_grants"."user_id", "role_grants"."resource_table", "role_grants"."resource_id" FROM "role_grants" WHERE "role_grants"."id" = $1 LIMIT $2"#,
                vec![grant.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "role_grants" WHERE "role_grants"."id" = $1"#,
                vec![grant.id.into()]
            )
        ]
    );

    Ok(())
}
