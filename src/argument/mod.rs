use chrono::NaiveDate;

mod date;
pub mod command;

#[derive(Debug)]
pub struct Arguments {
    pub command: command::Command,
    pub date: NaiveDate
}

pub fn get_arguments() -> Result<Arguments, String> {
    let mut arguments = std::env::args();
    arguments.next(); // Skip the first argument

    let potential_command = arguments.next();
    let command = command::get_command_from_args(potential_command)?;

    let potential_date = arguments.next();
    let date = date::get_date_from_args(potential_date)?;

    let args = Arguments {
        command,
        date: date.ok_or("A date argument is required")?
    };

    Ok(args)
}