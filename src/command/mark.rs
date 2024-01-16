use std::ffi::OsStr;
use chrono::{NaiveDate};
use inquire::{DateSelect, Editor, required, Text};
use tokio_postgres::Client;
use crate::config;

#[derive(Debug)]
struct InputMark {
    title: Option<String>,
    note: String,
    date: Option<NaiveDate>,
}

fn get_input_for_mark(should_provide_date_picker: bool) -> InputMark {
    let guard = config::CONFIG.lock().unwrap();
    let config = guard.as_ref().unwrap();
    let mark_style = &config.mark_style;
    let editor = &config.editor;

    let mut input_mark = match mark_style {
        config::MarkStyle::Default => {
            let _note = Text::new("Mark")
                .with_placeholder("Some text to be marked")
                .with_validator(required!("Mark text is required"))
                .with_help_message("Enter what you want to be marked")
                .prompt().unwrap();

            InputMark {
                title: None,
                note: _note,
                date: None,
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
                date: None,
            }
        },
    };

    match should_provide_date_picker {
        true => {
            let _date = DateSelect::new("Date")
                .with_help_message("Enter date for mark")
                .prompt().unwrap();

            input_mark.date = Some(_date);
        },
        false => {},
    }

    return input_mark;
}

pub async fn add_mark(client: &Client, should_provide_date_picker: bool) -> std::io::Result<()> {
    let input = get_input_for_mark(should_provide_date_picker);

    let statement = client.prepare("INSERT INTO marks (title, note, created_at) VALUES ($1, $2, $3) RETURNING id").await.expect("Could not prepare statement");

    let _title = &input.title.unwrap_or(String::new());
    let _created_at = &match input.date {
        Some(d) => { d.and_hms_opt(0, 0, 0).unwrap() },
        None => { chrono::Local::now().naive_local() }
    };

    client.query(&statement, &[_title, &input.note, _created_at]).await.expect("Could not execute query");

    Ok(())
}
