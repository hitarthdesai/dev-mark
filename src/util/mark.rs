use std::fmt::{Display, Formatter};
use tokio_postgres::{Row};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Mark {
    pub id: i8,
    title: String,
    pub note: String,
    pub created_at: NaiveDateTime,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", self.created_at.format("%H:%M").to_string(), self.note)
    }
}

impl Mark {
    pub fn new_from_row(row: &Row) -> Self {
        let id: i8 = i8::try_from(row.get::<&str, i64>("id")).unwrap();
        let title = row.get::<&str, &str>("title").to_string();
        let note = row.get::<&str, &str>("note").to_string();
        let created_at = row.get::<&str, NaiveDateTime>("created_at");

        Mark { id, title, note, created_at }
    }
}