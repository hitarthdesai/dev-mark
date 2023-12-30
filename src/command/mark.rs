use inquire::{required, Text};
use tokio_postgres::Client;

struct Mark {
    title: String,
    note: String,
}

fn get_input_for_mark() -> Mark {
    let _title = Text::new("Title")
        .with_placeholder("Title for new mark")
        .with_validator(required!("Title is required"))
        .with_help_message("Write what this mark is about")
        .prompt().unwrap();

    let _note = Text::new("Mark")
        .with_placeholder("Mark text")
        .with_validator(required!("Mark text is required"))
        .with_help_message("Enter what you want to be marked")
        .prompt().unwrap();

    return Mark {
        title: _title,
        note: _note,
    };
}

pub async fn add_mark(client: &Client) -> std::io::Result<()> {
    let input = get_input_for_mark();

    let statement = client.prepare("INSERT INTO marks (title, note) VALUES ($1, $2) RETURNING id").await.expect("Could not prepare statement");
    client.query(&statement, &[&input.title, &input.note]).await.expect("Could not execute query");

    Ok(())
}
