use tokio_postgres::{Error};

mod connect;

pub struct Database {
    pub client: tokio_postgres::Client,
}

pub async fn get_database() -> Result<Database, Error> {
    let client = connect::connect_to_db().await?;
    Ok(Database { client })
}
