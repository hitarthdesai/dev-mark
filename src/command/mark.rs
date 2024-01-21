use std::ffi::OsStr;
use chrono::{NaiveDate};
use inquire::{Editor, required, Text};
use crate::config;
use crate::db::Database;
use tokio_postgres::{Error};

#[derive(Debug)]
pub struct InputMark {
    pub title: Option<String>,
    pub note: String,
}

fn get_input_for_mark() -> InputMark {
    let guard = config::CONFIG.lock().unwrap();
    let config = guard.as_ref().unwrap();
    let mark_style = &config.mark_style;
    let editor = &config.editor;

    match mark_style {
        config::MarkStyle::Default => {
            let _note = Text::new("Mark")
                .with_placeholder("Some text to be marked")
                .with_validator(required!("Mark text is required"))
                .with_help_message("Enter what you want to be marked")
                .prompt().unwrap();

            InputMark {
                title: None,
                note: _note,
            }
        },

        config::MarkStyle::Extended => {
            let _title = Text::new("Title")
                .with_placeholder("Title for new mark")
                .with_validator(required!("Title is required"))
                .with_help_message("Write what this mark is about")
                .prompt().unwrap();

            let _note = Editor::new("Mark")
                .with_editor_command(OsStr::new(editor.as_str()))
                .with_validator(required!("Mark text is required"))
                .with_help_message("Enter what you want to be marked")
                .prompt().unwrap();

            InputMark {
                title: Some(_title),
                note: _note,
            }
        },
    }
}

pub async fn add_mark(db: &Database, date: NaiveDate) -> Result<(), Error> {
    let input = get_input_for_mark();
    let _title = &input.title.unwrap_or(String::new());
    let _created_at = &date.and_hms_opt(0, 0, 0).unwrap();

    db.add_mark(_created_at, _title, &input.note).await.expect("Could not execute query");

    Ok(())
}
