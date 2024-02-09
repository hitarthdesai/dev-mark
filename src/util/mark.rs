use std::fmt::{Display, Formatter};
use tokio_postgres::{Row};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Mark {
    pub id: i8,
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
        let note = row.get::<&str, &str>("note").to_string();
        let created_at = row.get::<&str, NaiveDateTime>("created_at");

        Mark { id, note, created_at }
    }
}