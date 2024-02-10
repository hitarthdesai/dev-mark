use inquire::{DateSelect, Text};
use chrono::{NaiveDate, NaiveTime};

mod date;
pub mod command;
mod time;

#[derive(Debug)]
pub struct Arguments {
    pub command: command::Command,
    pub date: NaiveDate,
    pub time: NaiveTime
}

pub fn get_arguments() -> Result<Arguments, String> {
    let mut arguments = std::env::args();
    arguments.next(); // Skip the first argument

    let potential_command = arguments.next();
    let command = command::get_command_from_args(potential_command)?;

    let remaining_args: Vec<String> = arguments.collect();
    let date_arg = remaining_args.iter().filter_map(|arg| {
        date::get_date_from_args(arg).ok()?
    }).last();

    /* TODO: Refactor the following code so that UI is not used in the argument module */
    let date = date_arg.unwrap_or_else(|| {
        DateSelect::new("Date").prompt().unwrap()
    });

    let time_arg = remaining_args.iter().filter_map(|arg| {
        time::get_time_from_args(arg).ok()?
    }).last();
    let time = time_arg.unwrap_or_else(|| {
        let time = Text::new("Time")
            .with_placeholder("Time for the event (HH::MM)")
            .prompt().unwrap();

        NaiveTime::parse_from_str(time.as_str(), "%R").expect("Invalid time")
    });

    let args = Arguments {
        command,
        date,
        time,
    };

    Ok(args)
}