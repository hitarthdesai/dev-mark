use std::fmt::{Display, Formatter};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Mark {
    pub id: i64,
    pub note: String,
    pub created_at: NaiveDateTime,
}

impl Display for Mark {
    /**
     * Format a mark for display in marks or unmark command
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", self.created_at.format("%H:%M").to_string(), self.note)
    }
}
