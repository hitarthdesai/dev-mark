use std::fmt::{Display, Formatter};
use chrono::{NaiveDate, NaiveDateTime};
use tokio_postgres::{Row, Error};
use crate::db::Database;

#[derive(Debug)]
pub struct Mark {
    pub id: i8,
    title: String,
    note: String,
    created_at: NaiveDateTime,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", self.created_at.format("%H:%M").to_string(), self.note)
    }
}

pub async fn transform_marks(rows: &Vec<Row>) -> Result<Vec<Mark>, Error> {
    let marks: Vec<Mark> = rows.iter().map(|row| {
        let id: i8 = i8::try_from(row.get::<&str, i64>("id")).unwrap();
        let title = row.get::<&str, &str>("title").to_string();
        let note = row.get::<&str, &str>("note").to_string();
        let created_at = row.get::<&str, NaiveDateTime>("created_at");

        return Mark { id, title, note, created_at }
    }).collect();

    Ok(marks)
}

pub async fn list_marks(db: &Database, date: &NaiveDate) -> Result<(), Error> {
    let _marks = db.read_marks_by_date(date).await?;
    let marks = transform_marks(&_marks).await?;

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
