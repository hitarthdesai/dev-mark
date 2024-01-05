mod command;
mod db;

#[tokio::main]
async fn main() {
    let client = db::connect::connect_to_db().await.expect("Could not connect to db");

    let mut arguments = std::env::args();
    arguments.next(); // Skip the first argument

    let command = arguments.next().expect("Command Expected");
    match command.as_str() {
        "mark" => {
            command::mark::add_mark(&client).await.expect("Could not add mark");
        },
        "marks" => {
            command::marks::list_marks(&client).await.expect("Could not read marks");
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
