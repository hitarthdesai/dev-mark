use chrono::{NaiveTime};

/**
 * Parse time from string
 *
 * This function parses a time from a string. The string can be in the format
 * "HH:MM" or "HH:MM:SS" or HH-MM or HH-MM-SS. The function returns a Result
 * with the parsed time or an error message.
 *
 * # Arguments
 * * `time` - A string with the time to be parsed
 *
 * # Returns
 * A Result with the parsed time or an error message
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