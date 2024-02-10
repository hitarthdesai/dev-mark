use tokio_postgres::{NoTls, Error, Client};

/**
 * Connect to the database
 */
pub async fn connect_to_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=15432 dbname=postgres",
        NoTls,
    )
        .await?;

    tokio::spawn(connection);

    Ok(client)
}