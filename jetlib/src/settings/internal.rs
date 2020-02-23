use config::{Config, ConfigError, File};
use crate::git::GitRepo;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalSettings {
    // On the first transition transition id is retrieved from the http API
    // After that we store ids in the internal config file to avoid unnecessary http
    // calls
    pub workflow: HashMap<String, String> // <transition name, transition id>,
}

impl ToString for InternalSettings {
    fn to_string(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}

impl Default for InternalSettings {
    fn default() -> Self {
        InternalSettings {
            workflow: Default::default()
        }
    }
}


impl InternalSettings {
    pub fn get() -> Result<Self, ConfigError> {
        let repo = GitRepo::open().unwrap();
        let workdir = repo.get_repo_dir().unwrap();

        let mut config_path = workdir.to_path_buf();
        config_path.push(".jet/config.internal.toml");

        if config_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(config_path))?;
            s.try_into()
        } else {
            Err(ConfigError::NotFound(
                "Unable to find .jet/config.internal.toml".into(),
            ))
        }
    }

    pub fn add_workflow(name: &str, id: &str) -> Result<Self, ConfigError> {
        let repo = GitRepo::open().unwrap();
        let workdir = repo.get_repo_dir().unwrap();

        let mut path = workdir.to_path_buf();

        // This could not fail, file was created with the get or create function
        path.push(".jet/config.internal.toml");

        let mut s = Config::new();
        s.merge(File::from(path))?;

        let mut workflow: HashMap<String, String> =
            if let Ok(workflow) = s.get("workflow") {
                workflow
            } else {
                HashMap::new()
            };

        let _ = workflow.insert(name.into(), id.into());
        s.set("workflow", workflow)?;
        s.try_into()
    }
}
