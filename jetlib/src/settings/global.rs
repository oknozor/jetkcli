use config::Config;
use config::ConfigError;
use config::File;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct GlobalSettings {
    pub servers: HashMap<String, Server>,
}

impl GlobalSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let home = env::var("HOME");
        if let Ok(home) = home {
            let config_path = format!("{}/.config/jet/config.toml", home);
            let mut s = Config::new();
            s.merge(File::with_name(&config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound("Unable to find home directory".into()))
        }
    }
}
