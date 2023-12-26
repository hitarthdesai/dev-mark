use inquire::{required, Text};
pub fn get_input_for_mark() -> String {
    let _title = Text::new("Title")
        .with_placeholder("Title for new mark")
        .with_validator(required!("Title is required"))
        .with_help_message("Write what this mark is about")
        .prompt().unwrap();

    let _mark = Text::new("Mark")
        .with_placeholder("Mark text")
        .with_validator(required!("Mark text is required"))
        .with_help_message("Enter what you want to be marked")
        .prompt().unwrap();

    return String::new();
}

// pub fn add_mark_to_file(mark_text: &str, file_path: &str) -> std::io::Result<()> {
//     // Get the current time in seconds since the Unix epoch
//     let timestamp = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards")
//         .as_secs();
//
//     // Format the mark with the timestamp
//     let mark_with_time = format!("{} - {}", timestamp, mark_text);
//
//     // Open the file in append mode or create it if it doesn't exist
//     let mut file = std::fs::OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(file_path).unwrap();
//
//     // Write the mark with time to the file
//     writeln!(file, "{}", mark_with_time).unwrap();
//
//     Ok(())
// }
