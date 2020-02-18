use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct ProjectSettings {
    pub id: i32,
    pub name: String,
    pub branch_types: Vec<String>,
    pub commit_types: Vec<String>,
    pub reviewers: Vec<String>,
    pub default_assignee: String,
}

impl ProjectSettings {
    pub fn new() -> Result<Self, ConfigError> {
        if let Ok(_) = std::env::current_dir() {
            let config_path = format!("./jet/config.toml");
            let mut s = Config::new();
            s.merge(File::with_name(&config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound("Unable to find jet config".into()))
        }
    }
}