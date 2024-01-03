use chrono::{DateTime, Local};
use tokio_postgres::Client;

#[derive(Debug)]
pub struct Mark {
    id: i8,
    title: String,
    note: String,
    created_at: DateTime<Local>,
}

pub async fn read_marks(client: &Client) -> std::io::Result<Vec<Mark>> {
    let statement = client.prepare("SELECT * FROM Marks ORDER BY created_at LIMIT 5").await.expect("Could not prepare statement");
    let _rows = client.query(&statement, &[]).await.expect("Could not execute query");

    let rows: Vec<Mark> = _rows.iter().map(|row| {
        let id: i8 = i8::try_from(row.get::<&str, i64>("id")).unwrap();
        let title = row.get::<&str, &str>("title").to_string();
        let note = row.get::<&str, &str>("note").to_string();
        let created_at = row.get::<&str, DateTime<Local>>("created_at");

        return Mark { id, title, note, created_at }
    }).collect();

    return Ok(rows)
}

pub async fn list_marks(client: &Client) -> std::io::Result<()> {
    let marks = read_marks(client).await.unwrap();

    marks.iter().for_each(|mark| {
        println!("On {}:\n{}\n", mark.created_at.format("%B%e, %Y at %H:%M").to_string(), mark.title);
    });

    Ok(())
}
