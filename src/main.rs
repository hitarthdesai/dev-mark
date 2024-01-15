use chrono::NaiveDate;
use regex::Regex;

mod command;
mod db;
mod config;

#[tokio::main]
async fn main() {
    config::initialize_config().expect("Could not read config");
    let client = db::connect::connect_to_db().await.expect("Could not connect to db");

    let mut arguments = std::env::args();
    arguments.next(); // Skip the first argument

    let command = arguments.next().expect("Command Expected");
    match command.as_str() {
        "mark" => {
            let should_provide_date_picker = match arguments.next() {
                Some(t) => {
                    match t.as_str() {
                        "--future" => { true },
                        _ => {
                            panic!("Error: Invalid argument. Allowed values: --future, or provide no args for today");
                        }
                    }
                },
                None => { false }
            };

            command::mark::add_mark(&client, should_provide_date_picker).await.expect("Could not add mark");
        },
        "marks" => {
            let mut _time_arg = arguments.next();
            let mut time_option: Option<NaiveDate> = None;

            match _time_arg {
                Some(_t) => {
                    let t = _t.as_str();
                    match Regex::new(r"--date=([^ ]+)").unwrap().captures(&t) {
                        None => {},
                        Some(c) => {
                            let date = c.get(1).unwrap().as_str();
                            time_option = Some(NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Invalid date format"));
                        }
                    }

                    match t {
                        "--today" => {
                            time_option = Some(chrono::Utc::now().date_naive());
                        },
                        _ => {
                            panic!("Error: Invalid argument. Allowed values: --today, --date=YYYY-MM-DD");
                        }
                    }
                },
                None => {
                    /* TODO: Allow to default to --today in config.json */
                    panic!("Error: No argument found. Allowed values: --today, --date=YYYY-MM-DD");
                }
            }

            command::marks::list_marks(&client, time_option).await.expect("Could not get marks");
        },
        "unmark" => {
            command::unmark::remove_mark(&client).await.expect("Could not remove mark");
        },
        _ => {
            // Invalid command, print an error message
            eprintln!("Error: Invalid command. Allowed values: mark");
        }
    }
}
