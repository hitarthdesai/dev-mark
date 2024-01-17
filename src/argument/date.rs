use chrono::NaiveDate;

pub fn get_date_from_args(_arg: String) -> Result<NaiveDate, &'static str> {
    let arg = _arg.as_str();

    if arg.starts_with("--today") {
        let mut anchor = chrono::Local::now().date_naive();
        let offset = arg.replace("--today", "");

        anchor += match offset.chars().next() {
            None => {
                chrono::Duration::days(0)
            },
            Some('+') => {
                let offset = offset[1..].parse::<i64>().expect("Invalid offset");
                chrono::Duration::days(offset)
            },
            Some('-') => {
                let offset = offset[1..].parse::<i64>().expect("Invalid offset");
                chrono::Duration::days(-offset)
            },
            Some(c) => {
                panic!("Invalid operator '{}'", c);
            }
        };

        Ok(anchor)
    } else if arg.starts_with("--date=") {
        let date = arg.replace("--date=", "");
        let parsed_date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").expect("Invalid date format, expected YYYY-MM-DD");
        Ok(parsed_date)
    } else {
        Err("Invalid argument")
    }
}
