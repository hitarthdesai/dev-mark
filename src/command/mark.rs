use chrono::NaiveDateTime;
use inquire::{required, Text};
use crate::db::Database;
use tokio_postgres::{Error};
use crate::argument::Arguments;

fn get_input_for_mark() -> String {
    Text::new("Mark")
        .with_placeholder("Some text to be marked")
        .with_validator(required!("Mark text is required"))
        .with_help_message("Enter what you want to be marked")
        .prompt().unwrap()
}

pub async fn add_mark(db: &Database, args: &Arguments) -> Result<(), Error> {
    let created_at = &NaiveDateTime::new(args.date, args.time);
    let note = get_input_for_mark();

    db.add_mark(created_at, &note).await.expect("Could not execute query");

    Ok(())
}
