use chrono::{NaiveTime};

/**
 * Takes in a string in the format of HH:MM or HH:MM:SS
 * or HH-MM or HH-MM-SS and returns a NaiveTime
 */
pub fn parse_time_from_string(time: &String) -> Result<NaiveTime, String> {
    let time = time.replace("-", ":");
    let num_parts = time.matches(":").count();

    match num_parts {
        1 => Ok(
            NaiveTime::parse_from_str(time.as_str(), "%R").unwrap()
        ),
        2 => Ok(
            NaiveTime::parse_from_str(time.as_str(), "%T").unwrap()
        ),
        _ => Err("Invalid time format".to_string())
    }
}