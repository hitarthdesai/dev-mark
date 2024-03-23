use inquire::DateSelect;
use chrono::{Duration, NaiveDate};
use crate::config::{CONFIG, DefaultDateTimeArg};


/**
 * Get the date from the command line arguments
 *
 * # Arguments
 *
 * * `arg` - The argument to parse
 *
 * # Returns
 *
 * An `Option` containing the date, or `None` if the argument is not a date
 */
pub fn get_date_from_args(arg: &String) -> Result<Option<NaiveDate>, &'static str> {
    let date_option = match arg.starts_with("--today") {
        false => { None }
        true => {
            let mut anchor = chrono::Local::now().date_naive();
            let offset = arg.replace("--today", "");

            anchor += match offset.chars().next() {
                None => {
                    Duration::days(0)
                },
                Some('+') => {
                    let offset = offset[1..].parse::<i64>().expect("Invalid offset");
                    Duration::days(offset)
                },
                Some('-') => {
                    let offset = offset[1..].parse::<i64>().expect("Invalid offset");
                    Duration::days(-offset)
                },
                Some(_) => {
                    return Err("Invalid operator");
                }
            };

            Some(anchor)
        },
    };

    Ok(date_option)
}

/**
 * Get the date from the user
 *
 * # Returns
 *
 * The date selected by the user
 */
fn get_date_from_user() -> NaiveDate {
    DateSelect::new("Date").prompt().unwrap()
}

/**
 * Get the date to use considering default date mode
 *
 * # Returns
 *
 * The date to use
 */
pub fn get_date() -> NaiveDate {
    let guard = CONFIG.lock().unwrap();
    let config = guard.as_ref().unwrap();
    let default_date_mode = &config.default_date;

    match default_date_mode {
        DefaultDateTimeArg::Current => {
            chrono::Local::now().date_naive()
        },
        DefaultDateTimeArg::Input => {
            get_date_from_user()
        },
    }
}
