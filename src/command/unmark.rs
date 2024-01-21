use chrono::NaiveDate;
use inquire::{Confirm, MultiSelect};
use inquire::list_option::ListOption;
use inquire::validator::Validation;
use tokio_postgres::Error;
use crate::command::marks::{Mark, transform_marks};
use crate::db::Database;

struct InputUnmark {
    ids: Vec<i64>,
    confirm: bool,
}

fn get_input_for_unmark(marks: Vec<Mark>) -> InputUnmark {
    let _selected_marks = MultiSelect::new("Select marks you no longer need", marks)
        .with_formatter(&|marks| format!("{} marks selected", marks.len()))
        .with_validator(&|marks: &[ListOption<&Mark>]| {
            match marks.len() >= 1 {
                true => Ok(Validation::Valid),
                false => Ok(Validation::Invalid("Select at least one mark".into())),
            }
        })
        .prompt()
        .unwrap();

    let _confirm_unmark = Confirm::new("Proceed with un-marking?")
        .with_default(false)
        .prompt()
        .unwrap();

    return InputUnmark {
        ids: _selected_marks.iter().map(|m| m.id.try_into().unwrap()).collect::<Vec<i64>>(),
        confirm: _confirm_unmark
    }
}

pub async fn remove_mark(db: &Database, date: &NaiveDate) -> Result<(), Error> {
    let _marks = db.read_marks_by_date(date).await?;
    if _marks.len() == 0 {
        println!("You have no marks for {}", date.format("%B %e, %Y").to_string());
        return Ok(());
    }

    let marks = transform_marks(&_marks).await?;

    println!("You marked the following on {}:", date.format("%B %e, %Y").to_string());
    let input = get_input_for_unmark(marks);

    if !input.confirm {
        return Ok(());
    }

    let statement = db.client.prepare("DELETE FROM marks WHERE id = $1").await.expect("Could not prepare statement");
    for id in input.ids {
        db.client.query(&statement, &[&id]).await.expect("Could not remove mark");
    }

    Ok(())
}
