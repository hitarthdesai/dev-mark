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

    let mut date: Option<NaiveDate> = None;
    for arg in arguments {
        if date.is_some() { break }
        match date::get_date_from_args(arg) {
            Ok(d) => { date = Some(d); },
            Err(_) => {}
        }
    }

    let args = Arguments {
        command,
        date: date.ok_or("A date argument is required")?
    };

    Ok(args)
}