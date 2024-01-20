pub mod date;
mod command;

pub fn get_arguments() -> Result<Vec<String>, String> {
    let mut arguments = std::env::args();
    arguments.next(); // Skip the first argument

    let potential_command = arguments.next();
    let command = command::get_command_from_args(potential_command)?;
    println!("{:?}", command);

    let args = arguments.collect::<Vec<String>>();

    Ok(args)
}