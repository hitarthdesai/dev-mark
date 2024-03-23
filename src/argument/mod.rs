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

    let date_arg = remaining_args.iter().filter_map(|arg| {
        date::get_date_from_args(arg).ok()?
    }).last();

    let time_arg = remaining_args.iter().filter_map(|arg| {
        time::get_time_from_args(arg).ok()?
    }).last();

    /* TODO: Refactor the following code so that UI is not used in the argument module */
    let date = date_arg.unwrap_or_else(date::get_date);

    /*
     * We only want to prompt the user for a time if the command is "mark",
     * otherwise return a default time
     */
    let time = time_arg.unwrap_or(match command {
        command::Command::Mark => time::get_time_from_user(),
        _ => NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    });

    let args = Arguments {
        command,
        date,
        time,
    };

    Ok(args)
}