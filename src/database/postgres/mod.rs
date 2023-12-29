use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

pub type Db = Arc<Pool<Postgres>>;

pub async fn db_connect() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Error connecting to database")
}
