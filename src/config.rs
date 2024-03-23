use std::fs;
use std::io::Error;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum DefaultDateTimeArg {
    Current,
    Input,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub connect_string: String,
    pub default_date: DefaultDateTimeArg,
    pub default_time: DefaultDateTimeArg,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

pub fn initialize_config() -> Result<(), Error> {
    let _config = fs::read_to_string("./config.json")?;
    let config: Config = serde_json::from_str(&_config)?;

    let mut guard = CONFIG.lock().unwrap();
    assert!( guard.is_none() );

    *guard = Some(config);
    Ok(())
}
