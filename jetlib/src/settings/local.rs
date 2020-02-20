use super::GLOBAL_SETTINGS;
use crate::git::GitRepo;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSettingsShared {
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
        // FIXME : handle no repo
        let repo = GitRepo::open().unwrap();
        let workdir = repo.get_repo_dir().unwrap();
        let mut config_path = workdir.to_path_buf();
        config_path.push(".jet/config.toml");

        if config_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(config_path))?;
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
            server_url,
            server_name: server_name.into(),
            project_name: project_name.into(),
            branch_types: vec![
                "fix".into(),
                "feat".into(),
                "chore".into(),
                "style".into(),
                "doc".into(),
            ],
            commit_types: vec![
                "fix".into(),
                "feat".into(),
                "chore".into(),
                "style".into(),
                "doc".into(),
            ],
            branch_separator: "/".into(),
        }
    }

    pub fn get() -> Result<Self, ConfigError> {
        // FIXME : handle no repo
        let repo = GitRepo::open().unwrap();
        let workdir = repo.get_repo_dir().unwrap();
        let mut config_path = workdir.to_path_buf();
        config_path.push(".jet/config.shared.toml");

        if config_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound(
                "Unable to find .jet/config.shared.toml".into(),
            ))
        }
    }
}
