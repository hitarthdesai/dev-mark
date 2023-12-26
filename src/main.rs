mod command;

fn main() {
    let mut arguments = std::env::args();
    arguments.next(); // Skip the first argument

    let command = arguments.next().expect("Command Expected");
    match command.as_str() {
        "mark" => {
            command::mark::get_input_for_mark();
        },
        "marks" => {
           command::marks::list_marks("marks.txt").expect("Could not read file");
        },
        "unmark" => {
            command::unmark::get_input_for_unmark();
        },
        _ => {
            // Invalid command, print an error message
            eprintln!("Error: Invalid command. Allowed values: mark");
        }
    }
}
