use tokio_postgres::{Error, Row};

mod connect;

pub struct Database {
    pub client: tokio_postgres::Client,
}

impl Database {
    /**
     * Add a mark to the database
     */
    pub async fn add_mark(&self, date: &chrono::NaiveDateTime, note: &String) -> Result<(), Error> {
        let statement = self.client.prepare("INSERT INTO marks (note, created_at) VALUES ($1, $2)").await?;
        self.client.query(&statement, &[note, date]).await?;

        Ok(())
    }

    /**
     * Get all marks for a date from the database
     */
    pub async fn read_marks_by_date(&self, date: &chrono::NaiveDate) -> Result<Vec<Row>, Error> {
        let statement = self.client.prepare("SELECT * FROM Marks WHERE DATE(created_at)=$1 ORDER BY created_at").await?;
        let rows: Vec<Row> = self.client.query(&statement, &[&date]).await?;

        Ok(rows)
    }

    /**
     * Delete mark with a specific id from the database
     */
    pub async fn delete_mark_by_id(&self, id: &i64) -> Result<(), Error> {
        let statement = self.client.prepare("DELETE FROM marks WHERE id = $1").await?;
        self.client.query(&statement, &[id]).await?;

        Ok(())
    }
}

/**
 * Get a database connection
 */
pub async fn get_database() -> Result<Database, Error> {
    let client = connect::connect_to_db().await?;
    Ok(Database { client })
}
