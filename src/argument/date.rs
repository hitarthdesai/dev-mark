use chrono::{Duration, NaiveDate};
use inquire::DateSelect;

enum DateOption {
    Past,
    Exact(NaiveDate),
    Future
}

pub fn get_date_from_args(_arg: Option<String>) -> Result<NaiveDate, &'static str> {
    let arg = match _arg {
        None => { return Err("No date option provided. Expected one of --past, --today, --future"); },
        Some(a) => { a.as_str() }
    };

    let date_option = match arg {
        "--past" => DateOption::Past,
        "--future" => DateOption::Future,
        _ => {
            if arg.starts_with("--today") {
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

                DateOption::Exact(anchor)
            } else {
                return Err("Invalid date option")
            }
        }
    };

    let date = match date_option {
        DateOption::Past => {
            DateSelect::new("Date")
                .with_max_date(chrono::Local::now().date_naive() - Duration::days(1))
                .with_help_message("Enter a past date for mark")
                .prompt().unwrap()
        },
        DateOption::Exact(date) => { date },
        DateOption::Future => {
            DateSelect::new("Date")
                .with_min_date(chrono::Local::now().date_naive() + Duration::days(1))
                .with_help_message("Enter a past date for mark")
                .prompt().unwrap()
        }
    };

    Ok(date)
}
