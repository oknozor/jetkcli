use super::GLOBAL_SETTINGS;
use crate::git::GitRepo;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSettingsShared {
    pub jira: JiraConfig,
    pub git: GitConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JiraConfig {
    pub project_name: String,
    pub server_name: String,
    pub server_url: String,
    pub workflow: Workflow,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitConfig {
    pub commit_types: Vec<String>,
    pub branch_types: Vec<String>,
    pub branch_separator: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub wip: String,
    pub done: String,
    pub todo: String,
}

impl Default for Workflow {
    fn default() -> Self {
        Workflow {
            todo: "To Do".to_string(),
            wip: "In Progress".to_string(),
            done: "Done".to_string(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        GitConfig {
            commit_types: vec![
                "fix".into(),
                "feat".into(),
                "chore".into(),
                "style".into(),
                "doc".into(),
            ],
            branch_types: vec![
                "fix".into(),
                "feat".into(),
                "chore".into(),
                "style".into(),
                "doc".into(),
            ],
            branch_separator: "/".into(),
        }
    }
}

impl ProjectSettingsShared {
    pub fn create(
        project_name: &str,
        server_name: &str,
    ) -> ProjectSettingsShared {
        let server_url = GLOBAL_SETTINGS.get_server_url(server_name).unwrap();
        let server_name = server_name.into();
        let project_name = project_name.into();

        ProjectSettingsShared {
            jira: JiraConfig {
                server_name,
                server_url,
                project_name,
                workflow: Default::default(),
            },
            git: Default::default(),
        }
    }

    pub fn get() -> Result<Self, ConfigError> {
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
