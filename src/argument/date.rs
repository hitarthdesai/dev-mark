use chrono::{Duration, NaiveDate};

pub fn get_date_from_args(arg: &String) -> Result<Option<NaiveDate>, &'static str> {
    let date_option = match arg.starts_with("--today") {
        false => { None }
        true => {
            let mut anchor = chrono::Local::now().date_naive();
            let offset = arg.replace("--today", "");

            anchor += match offset.chars().next() {
                None => {
                    Duration::days(0)
                },
                Some('+') => {
                    let offset = offset[1..].parse::<i64>().expect("Invalid offset");
                    Duration::days(offset)
                },
                Some('-') => {
                    let offset = offset[1..].parse::<i64>().expect("Invalid offset");
                    Duration::days(-offset)
                },
                Some(_) => {
                    return Err("Invalid operator");
                }
            };

            Some(anchor)
        },
    };

    Ok(date_option)
}
