use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn config_db() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error running a connection pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Error in running migrations");
    pool
}
