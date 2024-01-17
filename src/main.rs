mod command;
mod db;
mod config;
mod argument;

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
            let arg = arguments.next();
            let date = match arg {
                None => {
                    /* TODO: Allow to default to --today in config.json */
                    panic!("Error: No argument found. Allowed values: --today, --date=YYYY-MM-DD");
                },
                Some(t) => {
                    match argument::date::get_date_from_args(t) {
                        Ok(date) => { date },
                        Err(e) => {
                            panic!("Error: {}", e);
                        }
                    }
                }
            };

            command::marks::list_marks(&client, date).await.expect("Could not get marks");
        },
        "unmark" => {
            let arg = arguments.next();
            let date = match arg {
                None => {
                    /* TODO: Allow to default to --today in config.json */
                    panic!("Error: No argument found. Allowed values: --today, --date=YYYY-MM-DD");
                },
                Some(t) => {
                    match argument::date::get_date_from_args(t) {
                        Ok(date) => { date },
                        Err(e) => {
                            panic!("Error: {}", e);
                        }
                    }
                }
            };


            command::unmark::remove_mark(&client, date).await.expect("Could not remove mark");
        },
        _ => {
            // Invalid command, print an error message
            eprintln!("Error: Invalid command. Allowed values: mark");
        }
    }
}
