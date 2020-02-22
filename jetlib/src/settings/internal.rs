use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalSettings {
    workflow: Workflow,
}

// On the first transition transition id is retrieved from the http API
// After that we store ids in the internal config file to avoid unecessary http
// calls
#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub wip: i32,
    pub done: i32,
    pub todo: i32,
}

impl InternalSettings {
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
}
