#[derive(Debug)]
pub enum Command {
    Mark,
    Marks,
    Unmark,
}

pub fn get_command_from_args(arg: Option<String>) -> Result<Command, &'static str> {
    match arg {
        None => {
            Err("No command provided. Expected one of mark, marks, unmark")
        },
        Some(command) => {
            match command.as_str() {
                "mark" => {
                    Ok(Command::Mark)
                },
                "marks" => {
                    Ok(Command::Marks)
                },
                "unmark" => {
                    Ok(Command::Unmark)
                },
                _ => {
                    Err("Unexpected command. Expected one of mark, marks, unmark")
                }
            }
        }
    }
}