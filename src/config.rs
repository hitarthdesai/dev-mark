use std::env::VarError;
use std::sync::Mutex;
use dotenv::dotenv;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
pub enum DefaultDateTimeArg {
    Current,
    Input,
}

impl std::str::FromStr for DefaultDateTimeArg {
    type Err = VarError;

    fn from_str(s: &str) -> Result<Self, VarError> {
        match s {
            "current" => Ok(DefaultDateTimeArg::Current),
            "input" => Ok(DefaultDateTimeArg::Input),
            _ => Err(VarError::NotPresent),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub default_date: DefaultDateTimeArg,
    pub default_time: DefaultDateTimeArg,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

fn read_config_from_env_vars() -> Result<Config, VarError> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let default_date: DefaultDateTimeArg =  std::env::var("DEFAULT_DATE")?.parse()?;
    let default_time: DefaultDateTimeArg =  std::env::var("DEFAULT_TIME")?.parse()?;

    let config = Config {
        database_url,
        default_date,
        default_time,
    };

    Ok(config)
}

pub fn initialize_config() -> Result<(), VarError> {
    let mut guard = CONFIG.lock().unwrap();
    assert!( guard.is_none() );

    let config = read_config_from_env_vars()?;
    *guard = Some(config);

    Ok(())
}
