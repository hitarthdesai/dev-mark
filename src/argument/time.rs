use chrono::{NaiveTime};
use inquire::Text;

pub fn get_time_from_args(arg: &String) -> Result<Option<NaiveTime>, &'static str> {
    let time_option = match arg.starts_with("--time") {
        false => { None }
        true => {
            let arg = arg.replace("--time", "");
            if &arg[0..1] != "=" {
                return Err("Missing '=' after --time");
            }

            let offset = arg[1..].trim();

            /* TODO: Add ability to pars HH:MM:SS and HH:MM both */
            let time = NaiveTime::parse_from_str(offset, "%R").expect("Invalid time");
            Some(time)
        },
    };

    Ok(time_option)
}

pub fn get_time_from_user() -> NaiveTime {
    let time = Text::new("Time")
        .with_placeholder("Time for the mark (HH:MM)")
        .prompt().unwrap();

    NaiveTime::parse_from_str(time.as_str(), "%R").expect("Invalid time")
}