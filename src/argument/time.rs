use chrono::{NaiveTime};
use inquire::Text;
use crate::util::time::parse_time_from_string;

pub fn get_time_from_args(arg: &String) -> Result<Option<NaiveTime>, &'static str> {
    let time_option = match arg.starts_with("--time") {
        false => { None }
        true => {
            let arg = arg.replace("--time", "");
            if &arg[0..1] != "=" {
                return Err("Missing '=' after --time");
            }

            let offset = arg[1..].trim().to_string();

            /* TODO: Add ability to pars HH:MM:SS and HH:MM both */
            match parse_time_from_string(&offset) {
                Ok(time) => { Some(time) },
                Err(_) => { None }
            }
        },
    };

    Ok(time_option)
}

pub fn get_time_from_user() -> NaiveTime {
    let time = Text::new("Time")
        .with_placeholder("Time for the mark (HH:MM)")
        .prompt().unwrap();

    parse_time_from_string(&time.to_string()).expect("Could not parse time from user input")
}