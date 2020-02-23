use crate::{
    command::JetCommand,
    error::{ConfigAlreadyExist, JetError},
    git::GitRepo,
    jira::Jira,
    settings::{
        internal::InternalSettings,
        private::ProjectSettings,
        shared::ProjectSettingsShared,
        GLOBAL_SETTINGS,
    },
};
use std::{fs::File, io::Write, path::Path};

/// Init a .jetcli project inside a git repository
pub struct InitCommand {
    project_name: String,
    server_name: Option<String>,
}

impl JetCommand for InitCommand {
    fn execute(&self) -> Result<(), JetError> {
        // Use arg `--server` or the default one
        let server_name = if let Some(name) = &self.server_name {
            name
        } else {
            GLOBAL_SETTINGS.get_default_server_name()
        };

        let host = GLOBAL_SETTINGS.get_server_url(server_name).unwrap();
        let credentials = GLOBAL_SETTINGS.get_credentials(server_name);

        let jira = Jira {
            client: reqwest::Client::new(),
            credentials,
            host,
        };

        // Pre init check :
        // - is git repository ?
        // - is a jet project ?
        // - is a valid jira project ?
        let _ = GitRepo::open()?;
        InitCommand::maybe_init_already()?;
        let _ = jira.get_project(&self.project_name)?;

        // Get server from command or default value from config
        let settings = ProjectSettingsShared::create(&self.project_name, server_name);

        // Finally write to config
        std::fs::create_dir(".jet")?;
        let shared_settings = toml::to_string(&settings)?;
        let mut file = File::create(".jet/config.shared.toml")?;

        file.write_all(shared_settings.as_bytes())
            .map_err(JetError::from)?;

        let settings = ProjectSettings::create();
        let settings = toml::to_string(&settings)?;
        let mut file = File::create(".jet/config.toml")?;

        file.write_all(settings.as_bytes())
            .map_err(JetError::from)?;

        let internal = InternalSettings::default();
        let internal = toml::to_string(&internal)?;
        let mut file = File::create(".jet/config.internal.toml")?;

        file.write_all(internal.as_bytes()).map_err(JetError::from)
    }
}

impl InitCommand {
    pub fn new(
        project_name: &str,
        server_name: Option<&str>,
    ) -> InitCommand {
        InitCommand {
            project_name: project_name.to_string(),
            server_name: server_name.map(|opt| opt.into()),
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
