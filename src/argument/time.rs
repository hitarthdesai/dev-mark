use chrono::{NaiveDate, NaiveTime};
use inquire::Text;
use crate::config::{CONFIG, DefaultDateTimeArg};
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
fn get_time_from_user() -> NaiveTime {
    let time = Text::new("Time")
        .with_placeholder("Time for the mark (HH:MM)")
        .prompt().unwrap();

    parse_time_from_string(&time.to_string()).expect("Could not parse time from user input")
}

/**
 * Get the time to use considering default time mode
 *
 * # Returns
 *
 * The time to use
 */
pub fn get_time() -> NaiveTime {
    let guard = CONFIG.lock().unwrap();
    let config = guard.as_ref().unwrap();
    let default_time_mode = &config.default_time;

    match default_time_mode {
        DefaultDateTimeArg::Current => {
            chrono::Local::now().time()
        },
        DefaultDateTimeArg::Input => {
            get_time_from_user()
        },
    }
}