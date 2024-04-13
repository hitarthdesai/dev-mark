// use crate::config::CONFIG;
use sqlx::{SqlitePool, error::Error, Pool, Sqlite};

/**
 * Connect to the database
 */
pub async fn connect_to_db() -> Result<Pool<Sqlite>, Error> {
    // let guard = CONFIG.lock().unwrap();
    // let config = guard.as_ref().unwrap();
    // let connect_string = config.connect_string.as_str();

    let pool = SqlitePool::connect("sqlite:database.db").await?;
    Ok(pool)
}