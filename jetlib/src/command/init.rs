use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::error::JetError;
use git2::Repository;
use crate::command::JetCommand;
use crate::jira::Jira;
use crate::error::ConfigAlreadyExist;

/// Init a jet project inside a git repository
pub struct InitCommand {
    project_name: String,
    server_name: String,
}

impl JetCommand for InitCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError> {
        let _ = Repository::open(".")?;
        InitCommand::maybe_init_already()?;
        let _ = client.get_project(&self.project_name)?;
        self.write().map_err(|err| JetError::from(err))
    }
}

impl InitCommand {
    pub fn new(project_name: &str, server_name: &str) -> InitCommand {
        InitCommand {
            project_name: project_name.to_string(),
            server_name: server_name.to_string(),
        }
    }

    fn maybe_init_already() -> Result<(), ConfigAlreadyExist> {
        if Path::new("./.jet").exists() {
            Err(ConfigAlreadyExist)
        } else {
            Ok(())
        }
    }

    fn write(&self) -> Result<(), io::Error> {
        let mut file = File::create(".jet.toml")?;
        let config = format!(
            "server_name = \"{}\" \nproject_name= \"{}\""
            ,self.server_name, self.project_name
        );
        file.write_all(config.as_bytes())
    }
}