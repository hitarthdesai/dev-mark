use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mark_style: String,
    pub editor: String,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let _config = fs::read_to_string("./src/config.json")?;
    let config: Config = serde_json::from_str(&_config)?;

    println!("{:#?}", config);

    Ok(config)
}
