use anyhow::Result;
use async_graphql::MaybeUndefined::Undefined;
use fake::{Fake, Faker};
use nakago::Inject;
use nakago_axum::utils::ManyResponse;
use nakago_sea_orm::{connection, DatabaseConnection};
use pretty_assertions::assert_eq;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction, Value};

use crate::domains::{
    episodes::{
        model::Episode,
        mutation::{CreateEpisodeInput, UpdateEpisodeInput},
        query::{EpisodeCondition, EpisodesOrderBy},
        service::{self, Service},
    },
    shows::model::Show,
};

async fn setup(db: MockDatabase) -> nakago::Result<Inject> {
    let i = Inject::default();

    i.provide::<DatabaseConnection>(connection::ProvideMock::new(db))
        .await?;

    i.provide::<Box<dyn Service>>(service::Provide::default())
        .await?;

    Ok(i)
}

#[tokio::test]
async fn test_episodes_service_get_success() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = None;

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service.get(&episode.id, &false).await?;

    // Destroy the service to clean up the DB reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, Some(episode.clone()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
            vec![episode.id.into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_with_related() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = Some(show.clone());

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(episode.clone(), show.clone())]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service.get(&episode.id, &true).await?;

    // Destroy the service to clean up the DB reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, Some(episode.clone()));

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."id" = $1 LIMIT $2"#,
            vec![episode.id.into(), 1u64.into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = None;

    let mut other_show: Show = Faker.fake();
    other_show.title = "Test Show 2".to_string();

    let mut other_episode: Episode = Faker.fake();
    other_episode.title = "Test Episode 2".to_string();
    other_episode.show = None;

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone(), other_episode.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .get_many(
            Some(EpisodeCondition {
                title: Some("Test Episode".to_string()),
                show_id: None,
                ids_in: None,
            }),
            None,
            None,
            None,
            &false,
        )
        .await?;

    // Destroy the service to clean up the DB reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(
        result,
        ManyResponse {
            data: vec![episode, other_episode],
            count: 2,
            total: 2,
            page: 1,
            page_count: 1,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes" WHERE "episodes"."title" = $1"#,
            vec!["Test Episode".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many_with_related() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = Some(show.clone());

    let mut other_show: Show = Faker.fake();
    other_show.title = "Test Show 2".to_string();

    let mut other_episode: Episode = Faker.fake();
    other_episode.title = "Test Episode 2".to_string();
    other_episode.show = Some(other_show.clone());

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres).append_query_results(vec![vec![
            (episode.clone(), show.clone()),
            (other_episode.clone(), other_show.clone()),
        ]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .get_many(
            Some(EpisodeCondition {
                title: Some("Test Episode".to_string()),
                show_id: None,
                ids_in: None,
            }),
            None,
            None,
            None,
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(
        result,
        ManyResponse {
            data: vec![episode, other_episode],
            count: 2,
            total: 2,
            page: 1,
            page_count: 1,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."title" = $1"#,
            vec!["Test Episode".into()]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many_pagination() -> Result<()> {
    let mut show1: Show = Faker.fake();
    show1.title = "Test Show 1".to_string();

    let mut show2: Show = Faker.fake();
    show2.title = "Test Show 2".to_string();

    let mut show3: Show = Faker.fake();
    show3.title = "Test Show 3".to_string();

    let mut show4: Show = Faker.fake();
    show4.title = "Test Show 4".to_string();

    let mut show5: Show = Faker.fake();
    show5.title = "Test Show 5".to_string();

    let shows = vec![show1, show2, show3, show4, show5];

    let episodes: Vec<Episode> = shows
        .into_iter()
        .enumerate()
        .map(|(i, _)| {
            let mut episode: Episode = Faker.fake();
            episode.title = format!("Test Episode {}", i);
            episode.show = None;

            episode
        })
        .collect();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                episodes.clone(),
            ]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .get_many(
            None,
            Some(vec![EpisodesOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(
        result,
        ManyResponse {
            data: episodes,
            count: 5,
            total: 11,
            page: 2,
            page_count: 3,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes") AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes" ORDER BY "episodes"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_get_many_pagination_with_related() -> Result<()> {
    let mut show1: Show = Faker.fake();
    show1.title = "Test Show 1".to_string();

    let mut show2: Show = Faker.fake();
    show2.title = "Test Show 2".to_string();

    let mut show3: Show = Faker.fake();
    show3.title = "Test Show 3".to_string();

    let mut show4: Show = Faker.fake();
    show4.title = "Test Show 4".to_string();

    let mut show5: Show = Faker.fake();
    show5.title = "Test Show 5".to_string();

    let shows = vec![show1, show2, show3, show4, show5];

    let episodes: Vec<(Episode, Show)> = shows
        .into_iter()
        .enumerate()
        .map(|(i, show)| {
            let mut episode: Episode = Faker.fake();
            episode.title = format!("Test Episode {}", i);
            episode.show = Some(show.clone());

            (episode, show)
        })
        .collect();

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![maplit::btreemap! {
                // First query result
                "num_items" => Into::<Value>::into(11i64),
            }]])
            .append_query_results(vec![
                // Second query result
                episodes.clone(),
            ]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .get_many(
            None,
            Some(vec![EpisodesOrderBy::CreatedAtDesc]),
            Some(2),
            Some(5),
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(
        result,
        ManyResponse {
            data: episodes
                .into_iter()
                .map(|(episode, _show)| episode)
                .collect(),
            count: 5,
            total: 11,
            page: 2,
            page_count: 3,
        }
    );

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) AS num_items FROM (SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id") AS "sub_query""#,
                vec![]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" ORDER BY "episodes"."created_at" DESC LIMIT $1 OFFSET $2"#,
                vec![5u64.into(), 5u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_create() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show_id.clone_from(&show.id);
    episode.show = None;

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .create(
            &CreateEpisodeInput {
                title: episode.title.clone(),
                summary: episode.summary.clone(),
                picture: episode.picture.clone(),
                show_id: show.id.clone(),
            },
            &false,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, episode);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "episodes" ("title", "summary", "picture", "show_id") VALUES ($1, $2, $3, $4) RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "show_id""#,
            vec![
                episode.title.into(),
                episode.summary.into(),
                episode.picture.into(),
                episode.show_id.into(),
            ]
        )]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_create_with_related() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show_id.clone_from(&show.id);
    episode.show = Some(show.clone());

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .append_query_results(vec![vec![show.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .create(
            &CreateEpisodeInput {
                title: episode.title.clone(),
                summary: episode.summary.clone(),
                picture: episode.picture.clone(),
                show_id: show.id.clone(),
            },
            &true,
        )
        .await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    assert_eq!(result, episode);

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "episodes" ("title", "summary", "picture", "show_id") VALUES ($1, $2, $3, $4) RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "show_id""#,
                vec![
                    episode.title.into(),
                    episode.summary.into(),
                    episode.picture.into(),
                    episode.show_id.into(),
                ]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "shows"."id", "shows"."created_at", "shows"."updated_at", "shows"."title", "shows"."summary", "shows"."picture" FROM "shows" WHERE "shows"."id" = $1 LIMIT $2"#,
                vec![show.id.into(), 1u64.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_update() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = None;

    let updated = Episode {
        title: "Updated Episode".to_string(),
        ..episode.clone()
    };

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()], vec![updated.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .update(
            &show.id,
            &UpdateEpisodeInput {
                title: Some(updated.title.clone()),
                summary: Undefined,
                picture: Undefined,
                show_id: Some(show.id.clone()),
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
                r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "episodes" SET "title" = $1, "show_id" = $2 WHERE "episodes"."id" = $3 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "show_id""#,
                vec![updated.title.into(), show.id.into(), episode.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_update_with_related() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = Some(show.clone());

    let updated = Episode {
        title: "Updated Episode".to_string(),
        ..episode.clone()
    };

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(episode.clone(), show.clone())]])
            .append_query_results(vec![vec![updated.clone()]]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    let result = service
        .update(
            &show.id,
            &UpdateEpisodeInput {
                title: Some(updated.title.clone()),
                summary: Undefined,
                picture: Undefined,
                show_id: Some(show.id.clone()),
            },
            &true,
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
                r#"SELECT "episodes"."id" AS "A_id", "episodes"."created_at" AS "A_created_at", "episodes"."updated_at" AS "A_updated_at", "episodes"."title" AS "A_title", "episodes"."summary" AS "A_summary", "episodes"."picture" AS "A_picture", "episodes"."show_id" AS "A_show_id", "shows"."id" AS "B_id", "shows"."created_at" AS "B_created_at", "shows"."updated_at" AS "B_updated_at", "shows"."title" AS "B_title", "shows"."summary" AS "B_summary", "shows"."picture" AS "B_picture" FROM "episodes" LEFT JOIN "shows" ON "episodes"."show_id" = "shows"."id" WHERE "episodes"."id" = $1 LIMIT $2"#,
                vec![show.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"UPDATE "episodes" SET "title" = $1, "show_id" = $2 WHERE "episodes"."id" = $3 RETURNING "id", "created_at", "updated_at", "title", "summary", "picture", "show_id""#,
                vec![updated.title.into(), show.id.into(), episode.id.into()]
            )
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_episodes_service_delete() -> Result<()> {
    let mut show: Show = Faker.fake();
    show.title = "Test Show".to_string();

    let mut episode: Episode = Faker.fake();
    episode.title = "Test Episode".to_string();
    episode.show = None;

    let i = setup(
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![episode.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }]),
    )
    .await?;

    let service = i.get::<Box<dyn Service>>().await?;

    service.delete(&episode.id).await?;

    // Destroy the service to clean up the reference count
    drop(service);

    let db = i.eject::<DatabaseConnection>().await?;

    // Check the transaction log
    assert_eq!(
        db.into_transaction_log(),
        vec![
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,
                vec![episode.id.clone().into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"DELETE FROM "episodes" WHERE "episodes"."id" = $1"#,
                vec![episode.id.into()]
            )
        ]
    );

    Ok(())
}
