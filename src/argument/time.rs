use chrono::{NaiveTime};

pub fn get_time_from_args(arg: &String) -> Result<Option<NaiveTime>, &'static str> {
    let time_option = match arg.starts_with("--time") {
        false => { None }
        true => {
            let offset = arg.replace("--time", "");
            if offset.is_empty() {
                return Err("Missing '=HH:MM:SS' after --time");
            }

            let time = offset.parse::<NaiveTime>().expect("Invalid time");
            Some(time)
        },
    };

    Ok(time_option)
}
