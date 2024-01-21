use chrono::{NaiveDate};
use tokio_postgres::{Error};
use crate::db::Database;
use crate::util::mark::Mark;

pub async fn list_marks(db: &Database, date: &NaiveDate) -> Result<(), Error> {
    let _marks = db.read_marks_by_date(date).await?;
    let marks: Vec<Mark> = _marks.iter().map(Mark::new_from_row).collect();

    if marks.len() == 0 {
        println!("You have no marks for {}", date.format("%B %e, %Y").to_string());
        return Ok(());
    }


    println!("On {}, you marked:", date.format("%B %e, %Y").to_string());
    marks.iter().for_each(|mark| {
        println!("{}: {}\n", mark.created_at.format("%H:%M").to_string(), mark.note);
    });

    Ok(())
}
