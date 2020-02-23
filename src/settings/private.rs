use crate::git::GitRepo;
use config::{Config, ConfigError, File};

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
