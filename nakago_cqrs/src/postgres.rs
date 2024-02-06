use nakago::Tag;
use sqlx::{Pool, Postgres};

/// The PostgresPool Tag
pub const POSTGRES_POOL: Tag<Pool<Postgres>> = Tag::new("postgres::Pool");
