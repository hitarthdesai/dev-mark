use tokio_postgres::{NoTls, Error, Client};
use crate::config::CONFIG;

/**
 * Connect to the database
 */
pub async fn connect_to_db() -> Result<Client, Error> {
    let guard = CONFIG.lock().unwrap();
    let config = guard.as_ref().unwrap();
    let connect_string = config.connect_string.as_str();

    let (client, connection) = tokio_postgres::connect(
        connect_string,
        NoTls,
    )
        .await?;

    tokio::spawn(connection);

    Ok(client)
}