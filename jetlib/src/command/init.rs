use crate::command::{JetJiraCommand, JetCommand};
use crate::error::ConfigAlreadyExist;
use crate::error::JetError;
use crate::jira::Jira;
use crate::settings::GLOBAL_SETTINGS;
use git2::Repository;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use crate::settings::local::{ProjectSettingsShared, ProjectSettings};
use std::borrow::Borrow;

/// Init a .jetcli project inside a git repository
pub struct InitCommand {
    project_name: String,
    server_name: Option<String>,
}

impl JetCommand for InitCommand {
    fn execute(&self) -> Result<(), JetError> {
        let server_name = if let Some(name) = &self.server_name {
            name
        } else {
            GLOBAL_SETTINGS.get_default_server_name()
        };

        let host = GLOBAL_SETTINGS.get_server_url(server_name).unwrap();
        let credentials = GLOBAL_SETTINGS.get_credentials(server_name);

        let mut jira = Jira {
            client: reqwest::Client::new(),
            credentials,
            host,
        };

        // Pre init check :
        // - is git repository ?
        // - is a jet project ?
        // - is a valid jira project ?
        let _ = Repository::open(".")?;
        InitCommand::maybe_init_already()?;
        let _ = jira.get_project(&self.project_name)?;

        // Get server from command or default value from config
        let settings = ProjectSettingsShared::create(&self.project_name, server_name);

        // Finally write to config
        let shared_settings = toml::to_string(&settings)?;
        let mut file = File::create(".jet/config.shared.toml")?;
        file.write_all(shared_settings.as_bytes())
            .map_err(|err| JetError::from(err))?;

        let settings = ProjectSettings::create();
        let settings = toml::to_string(&settings)?;
        let mut file = File::create(".jet/config.toml")?;
        file.write_all(settings.as_bytes())
            .map_err(|err| JetError::from(err))
    }
}

impl InitCommand {
    pub fn new(project_name: &str, server_name: &str) -> InitCommand {
        InitCommand {
            project_name: project_name.to_string(),
            server_name: Some(server_name.to_string()),
        }
    }

    pub fn new_default_server(project_name: &str) -> InitCommand {
        InitCommand {
            project_name: project_name.to_string(),
            server_name: None,
        }
    }

    fn maybe_init_already() -> Result<(), ConfigAlreadyExist> {
        if Path::new("./.jet/config.shared.toml").exists() {
            Err(ConfigAlreadyExist {})
        } else {
            Ok(())
        }
    }
}
