use chrono::{NaiveTime};
use inquire::Text;
use crate::util::time::parse_time_from_string;

/**
 * Get the time from the command line arguments
 *
 * # Arguments
 *
 * * `arg` - The argument to parse
 *
 * # Returns
 *
 * An `Option` containing the time, or `None` if the argument is not a time
 */
pub fn get_time_from_args(arg: &String) -> Result<Option<NaiveTime>, &'static str> {
    let time_option = match arg.starts_with("--time") {
        false => { None }
        true => {
            let arg = arg.replace("--time", "");
            if &arg[0..1] != "=" {
                return Err("Missing '=' after --time");
            }

            let offset = arg[1..].trim().to_string();

            match parse_time_from_string(&offset) {
                Ok(time) => { Some(time) },
                Err(_) => { None }
            }
        },
    };

    Ok(time_option)
}

/**
 * Get the time from the user
 *
 * # Returns
 *
 * The time selected by the user
 */
pub fn get_time_from_user() -> NaiveTime {
    let time = Text::new("Time")
        .with_placeholder("Time for the mark (HH:MM)")
        .prompt().unwrap();

    parse_time_from_string(&time.to_string()).expect("Could not parse time from user input")
}