use chrono::NaiveDate;

mod date;
pub mod command;
mod time;

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

    let remaining_args: Vec<String> = arguments.collect();
    let date_arg = remaining_args.iter().filter_map(|arg| {
        date::get_date_from_args(arg)?
    }).last();
    let time_arg = remaining_args.iter().filter_map(|arg| {
        time::get_time_from_args(arg)?
    }).last();

    let args = Arguments {
        command,
        date: date?,
    };

    Ok(args)
}