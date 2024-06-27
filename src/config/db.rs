use dotenv::dotenv;
use log::info;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub async fn config_db() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to database at {}", database_url);
    let pool = PgPool::connect(&database_url).await?;

    // Optionally, uncomment this line if you have migrations to run
    // sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
