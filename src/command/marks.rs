use std::fmt::{Display, Formatter};
use chrono::{NaiveDate, NaiveDateTime};
use tokio_postgres::{Client, Row};

#[derive(Debug)]
pub struct Mark {
    pub id: i8,
    title: String,
    note: String,
    created_at: NaiveDateTime,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on {}", self.title, self.created_at.format("%B%e, %Y at %H:%M"))
    }
}

pub async fn read_marks(client: &Client, time_frame: Option<NaiveDate>) -> std::io::Result<Vec<Mark>> {
    let _rows: Vec<Row> = match time_frame {
        None => {
            let statement = client.prepare("SELECT * FROM Marks ORDER BY created_at").await.expect("Could not prepare statement");
            client.query(&statement, &[]).await.expect("Could not execute query")
        },
        Some(t) => {
            let statement = client.prepare("SELECT * FROM Marks WHERE DATE(created_at)=$1 ORDER BY created_at").await.expect("Could not prepare statement");
            client.query(&statement, &[&t]).await.expect("Could not execute query")
        }
    };

    let rows: Vec<Mark> = _rows.iter().map(|row| {
        let id: i8 = i8::try_from(row.get::<&str, i64>("id")).unwrap();
        let title = row.get::<&str, &str>("title").to_string();
        let note = row.get::<&str, &str>("note").to_string();
        let created_at = row.get::<&str, NaiveDateTime>("created_at");

        return Mark { id, title, note, created_at }
    }).collect();

    return Ok(rows)
}

pub async fn list_marks(client: &Client, time_frame: Option<NaiveDate>) -> std::io::Result<()> {
    let marks = read_marks(client, time_frame).await.unwrap();

    marks.iter().for_each(|mark| {
        println!("On {}:\n{}\n", mark.created_at.format("%B %e, %Y at %H:%M").to_string(), mark.note);
    });

    Ok(())
}
