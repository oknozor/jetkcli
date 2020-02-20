use crate::command::JetCommand;
use crate::error::JetError;
use crate::git::GitRepo;
use crate::settings::global::GlobalSettings;
use crate::settings::local::{ProjectSettings, ProjectSettingsShared};

pub struct InfoCommand;

impl JetCommand for InfoCommand {
    fn execute(&self) -> Result<(), JetError> {
        println!("Jet Project info  :");

        match ProjectSettingsShared::get() {
            Ok(ref settings) => {
                println!("Shared settings \n{}", toml::to_string(settings).unwrap())
            }
            Err(e) => eprintln!("{}", JetError::from(e)),
        };

        match ProjectSettings::get() {
            Ok(ref settings) => {
                println!("Personal settings \n{}", toml::to_string(settings).unwrap())
            }
            Err(e) => eprintln!("{}", JetError::from(e)),
        };

        match GlobalSettings::get() {
            Ok(ref settings) => {
                println!("Global settings \n{}", toml::to_string(settings).unwrap())
            }
            Err(e) => eprintln!("{}", JetError::from(e)),
        };

        match GitRepo::open() {
            Ok(repo) => println!(
                "Working on repository {}",
                repo.get_repo_dir().unwrap().display()
            ),
            Err(e) => eprintln!("{}", JetError::from(e)),
        }

        Ok(())
    }
}
