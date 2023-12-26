mod command;

fn main() {
    let mut arguments = std::env::args();

    let command = arguments.nth(1).expect("Command Expected");

    match command.as_str() {
        "mark" => {
            let _input_text: Vec<String> = arguments.collect();
            let input_text = _input_text.join(" ");

            command::mark::add_mark_to_file(&input_text, "marks.txt").expect("Could not write to file");
        },
        "marks" => {
           command::marks::list_marks("marks.txt").expect("Could not read file");
        },
        "unmark" => {
            command::unmark::start_interactive_session("marks.txt").expect("Could not read file");
        },
        _ => {
            // Invalid command, print an error message
            eprintln!("Error: Invalid command. Allowed values: mark");
        }
    }
}
