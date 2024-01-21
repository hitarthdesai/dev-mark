mod command;
mod db;
mod config;
mod argument;

#[tokio::main]
async fn main() {
    config::initialize_config().expect("Could not read config");
    let db = db::get_database().await.expect("Could not connect to database");

    let args = match argument::get_arguments() {
        Ok(args) => { args },
        Err(e) => { panic!("Error: {}", e) }
    };

    match args.command {
        argument::command::Command::Mark => {
            command::mark::add_mark(&db, args.date).await.expect("Could not add mark");
        },
        argument::command::Command::Marks => {
            command::marks::list_marks(&db, &args.date).await.expect("Could not get marks");
        },
        argument::command::Command::Unmark => {
            command::unmark::remove_mark(&db, &args.date).await.expect("Could not remove mark");
        },
    }
}