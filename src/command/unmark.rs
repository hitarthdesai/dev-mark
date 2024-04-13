use inquire::{Confirm, MultiSelect};
use inquire::list_option::ListOption;
use inquire::validator::Validation;
use sqlx::Error;
use crate::argument::Arguments;
use crate::db::Database;
use crate::util::mark::Mark;

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

pub async fn remove_mark(db: &Database, args: &Arguments) -> Result<(), Error> {
    let marks = db.read_marks_by_date(&args.date).await?;
    if marks.len() == 0 {
        println!("You have no marks for {}", &args.date.format("%B %e, %Y").to_string());
        return Ok(());
    }

    println!("You marked the following on {}:", &args.date.format("%B %e, %Y").to_string());
    let input = get_input_for_unmark(marks);

    if !input.confirm {
        return Ok(());
    }

    for id in input.ids.iter() {
        db.delete_mark_by_id(id).await?;
    }

    Ok(())
}
