use tokio_postgres::{Error};

mod connect;

pub struct Database {
    pub client: tokio_postgres::Client,
}

impl Database {
    pub async fn add_mark(&self, date: &chrono::NaiveDateTime, title: &String, note: &String) -> Result<(), Error> {
        let statement = self.client.prepare("INSERT INTO marks (title, note, created_at) VALUES ($1, $2, $3) RETURNING id").await?;

        self.client.query(&statement, &[title, note, date]).await?;

        Ok(())
    }
}

pub async fn get_database() -> Result<Database, Error> {
    let client = connect::connect_to_db().await?;
    Ok(Database { client })
}
