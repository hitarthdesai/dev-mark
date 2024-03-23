use chrono::{NaiveDate, NaiveTime};

mod date;
pub mod command;
mod time;

/**
 * The arguments that are passed to the program
 */
#[derive(Debug)]
pub struct Arguments {
    pub command: command::Command,
    pub date: NaiveDate,
    pub time: NaiveTime
}

/**
 * Get the arguments from the command line
 *
 * # Returns
 *
 * An `Arguments` struct containing the command, date, and time
 *
 * # Errors
 *
 * If the command is not recognized, or if the date or time is not in the correct format
 */
pub fn get_arguments() -> Result<Arguments, String> {
    let mut arguments = std::env::args();
    arguments.next(); /* Skip the first argument */

    let potential_command = arguments.next();
    let command = command::get_command_from_args(potential_command)?;

    let remaining_args: Vec<String> = arguments.collect();
    let date = date::get_date(&remaining_args);
    let time = time::get_time(&remaining_args);

    let args = Arguments {
        command,
        date,
        time,
    };

    Ok(args)
}