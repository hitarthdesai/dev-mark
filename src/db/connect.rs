use crate::config::CONFIG;
use sqlx::{SqlitePool, error::Error, Pool, Sqlite};

/**
 * Connect to the database
 */
pub async fn connect_to_db() -> Result<Pool<Sqlite>, Error> {
    let guard = CONFIG.lock().unwrap();
    let config = guard.as_ref().unwrap();
    let database_url = config.database_url.as_str();


    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}