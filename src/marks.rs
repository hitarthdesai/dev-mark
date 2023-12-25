use std::io::{Read};
use chrono::{Utc, TimeZone};

fn _format_marks(marks: &str) -> String {
    let mut formatted_marks = String::new();

    for mark in marks.lines() {
        let mark = mark.trim();
        let mark = mark.split(" - ").collect::<Vec<&str>>();

        match mark.len() {
            2 => {
                let _timestamp = mark[0].parse::<i64>().unwrap();
                let formatted_time_stamp = Utc
                    .timestamp_opt(_timestamp, 0)
                    .unwrap()
                    .format("On %B %e, %Y at %H:%M:%S").to_string();
                let mark = mark[1];

                formatted_marks.push_str(&format!("{} - {}\n", formatted_time_stamp, mark));
            },
            _ => { eprintln!("Invalid mark: {}", mark.join(" - ")); },
        };
    }

    formatted_marks
}

pub fn list_marks(file_path: &str) -> std::io::Result<()> {
    // Format the mark with the timestamp

    // Open the file in append mode or create it if it doesn't exist
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", _format_marks(contents.as_str()));

    Ok(())
}
