use sqlx::{Error, Pool, Sqlite};
use crate::util::mark::Mark;

mod connect;

pub struct Database {
    pub pool: Pool<Sqlite>
}

impl Database {
    /**
     * Add a mark to the database
     */
    pub async fn add_mark(&self, date: &chrono::NaiveDateTime, note: &String) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO marks (note, created_at) VALUES (?, ?)",
            note,
            date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /**
     * Get all marks for a date from the database
     */
    pub async fn read_marks_by_date(&self, date: &chrono::NaiveDate) -> Result<Vec<Mark>, Error> {
        let rows = sqlx::query_as!(
            Mark,
            "SELECT * FROM Marks WHERE DATE(created_at)=? ORDER BY created_at",
            date
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /**
     * Delete mark with a specific id from the database
     */
    pub async fn delete_mark_by_id(&self, id: &i64) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM marks WHERE id=?",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

/**
 * Get a database connection
 */
pub async fn get_database() -> Result<Database, Error> {
    let pool = connect::connect_to_db().await?;
    Ok(Database { pool })
}
