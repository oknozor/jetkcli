use std::collections::HashMap;

use config::Config;
use config::ConfigError;
use config::File;
use dirs;

use crate::jira::Credentials;

use super::PROJECT_SETTINGS_SHARED;
use std::path::PathBuf;

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
    pub fn get() -> Result<Self, ConfigError> {
        let path = Self::path();

        if path.exists() {
            let mut s = Config::new();
            s.merge(File::from(path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound(
                "Unable to find home directory".into(),
            ))
        }
    }

    fn path() -> PathBuf {
        let mut config_path = dirs::config_dir()
            .unwrap_or_else(|| panic!("unable to local XDG_CONFIG directory!"));
        config_path.push(".jet");
        config_path.push("config.toml");
        config_path
    }
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
}

impl Server {
    fn as_credentials(&self) -> Credentials {
        Credentials {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

