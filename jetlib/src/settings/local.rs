use config::{Config, ConfigError, File};
use std::path::Path;
use crate::settings::global::GlobalSettings;
use super::GLOBAL_SETTINGS;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSettingsShared {
    pub id: i32,
    pub project_name: String,
    pub server_name: String,
    pub server_url: String,
    pub branch_types: Vec<String>,
    pub commit_types: Vec<String>,
    pub branch_separator: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSettings {
    pub reviewers: Vec<String>,
    pub default_assignee: String,
}

impl ProjectSettings {
    pub fn create() -> ProjectSettings {
        ProjectSettings {
            reviewers: vec![],
            default_assignee: "".to_string(),
        }
    }

    pub fn get() -> Result<Self, ConfigError> {
        let config_path = ".jet/config.toml";
        if Path::new(config_path).exists() {
            let mut s = Config::new();
            s.merge(File::with_name(config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound(
                "Unable to find .jet/config.toml".into(),
            ))
        }
    }
}

impl ProjectSettingsShared {
    pub fn create(project_name: &str, server_name: &str) -> ProjectSettingsShared {
        let server_url = GLOBAL_SETTINGS.get_server_url(server_name).unwrap();
        ProjectSettingsShared {
            id: 0,
            server_url,
            server_name: server_name.into(),
            project_name: project_name.into(),
            branch_types: vec!["fix".into(), "feat".into(), "chore".into(), "style".into(), "doc".into()],
            commit_types: vec!["fix".into(), "feat".into(), "chore".into(), "style".into(), "doc".into()],
            branch_separator: "/".into(),
        }
    }

    pub fn get() -> Result<Self, ConfigError> {
        let config_path = ".jet/config.shared.toml";
        if Path::new(config_path).exists() {
            let mut s = Config::new();
            s.merge(File::with_name(config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound(
                "Unable to find .jet/config.shared.toml".into(),
            ))
        }
    }
}
