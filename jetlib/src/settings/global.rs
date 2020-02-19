use std::collections::HashMap;
use std::env;

use config::Config;
use config::ConfigError;
use config::File;

use crate::jira::Credentials;

use super::PROJECT_SETTINGS_SHARED;

#[derive(Debug, Deserialize)]
pub struct GlobalSettings {
    pub servers: HashMap<String, Server>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl GlobalSettings {
    pub fn get_default_server_url(&self) -> &str {
        &self.servers.iter().last().unwrap().1.url
    }

    pub fn get_default_server_name(&self) -> &str {
        &self.servers.keys().last().unwrap()
    }

    pub fn get_server_url(&self, key: &str) -> Option<String> {
        self.servers.get(key).map(|server| server.url.clone())
    }

    pub fn get_credentials(&self, server_name: &str) -> Credentials {
        self.servers.get(server_name).unwrap().as_credentials()
    }

    pub fn current_credentials(&self) -> Credentials {
        let server_name = PROJECT_SETTINGS_SHARED.server_name.clone();
        self.servers.get(&server_name).unwrap().as_credentials()
    }

    pub fn get() -> Result<Self, ConfigError> {
        let home = env::var("HOME");
        if let Ok(home) = home {
            let config_path = format!("{}/.config/jet/config.toml", home);
            let mut s = Config::new();
            s.merge(File::with_name(&config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound(
                "Unable to find home directory".into(),
            ))
        }
    }
}

impl Server {
    fn as_credentials(&self) -> Credentials {
        Credentials {
            username: self.username.clone(),
            password: self.username.clone(),
        }
    }
}

