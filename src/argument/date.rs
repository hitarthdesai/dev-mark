use chrono::{Duration, NaiveDate};
use inquire::DateSelect;

enum DateOption {
    Custom,
    Exact(NaiveDate),
}

pub fn get_date_from_args(_arg: Option<String>) -> Result<NaiveDate, &'static str> {
    let date_option = match _arg {
        None => { DateOption::Custom },
        Some(a) => {
            match a.starts_with("--today") {
                true => {
                    let mut anchor = chrono::Local::now().date_naive();
                    let offset = a.replace("--today", "");

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

                    DateOption::Exact(anchor)
                },
                false => {
                    DateOption::Custom
                }
            }
        }
    };

    let date = match date_option {
        DateOption::Exact(date) => { date },
        DateOption::Custom => {
            DateSelect::new("Date")
                .with_help_message("Enter a date for mark")
                .prompt().unwrap()
        }
    };

    Ok(date)
}
