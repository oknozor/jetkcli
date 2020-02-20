use std::error::Error;
use std::fmt;
use std::{env, io};
use config::ConfigError;

#[derive(Debug)]
pub enum JetError {
    FileNotFound(io::Error),
    ConfigError(ConfigError),
    ConfigAlreadyExist(ConfigAlreadyExist),
    NotAGitRepository(git2::Error),
    EmptyIndex,
    JiraResourceNotFound(reqwest::Error),
    TomlError(toml::ser::Error),
    Other,
}

impl fmt::Display for JetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JetError::ConfigAlreadyExist(ref cause) => {
                write!(f, "Config already exist : {}", cause)
            }
            JetError::FileNotFound(ref cause) => write!(f, "File not found : {}", cause),
            JetError::NotAGitRepository(ref cause) => write!(f, "Current dir is not a git repository : {}", cause),
            JetError::EmptyIndex => write!(f, "nothing added to commit but untracked files present (use \"git add\" to track)"),
            JetError::JiraResourceNotFound(ref cause) => write!(f, "Error fetching resource from jira : {}", cause),
            JetError::TomlError(ref cause) => write!(f, "Error during config serialization: {}", cause),
            JetError::Other => write!(f, "Unknown Jet error"),
            JetError::ConfigError(ref cause) => write!(f, "Config error {}", cause),
        }
    }
}

impl Error for JetError {
    fn description(&self) -> &str {
        match *self {
            JetError::FileNotFound(ref cause) => cause.description(),
            JetError::ConfigAlreadyExist(ref cause) => cause.description(),
            JetError::NotAGitRepository(ref cause) => cause.description(),
            JetError::EmptyIndex => "nothing added to commit but untracked files present (use \"git add\" to track)",
            JetError::JiraResourceNotFound(ref cause) => cause.description(),
            JetError::TomlError(ref cause) => cause.description(),
            JetError::Other => "Unknown .jetcli error!",
            JetError::ConfigError(ref cause) => cause.description()
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            JetError::FileNotFound(ref cause) => Some(cause),
            JetError::ConfigAlreadyExist(ref cause) => Some(cause),
            JetError::NotAGitRepository(ref cause) => Some(cause),
            JetError::JiraResourceNotFound(ref cause) => Some(cause),
            JetError::TomlError(ref cause) => Some(cause),
            JetError::Other => None,
            JetError::ConfigError(ref cause) => Some(cause),
            JetError::EmptyIndex => None,
        }
    }
}

impl From<io::Error> for JetError {
    fn from(cause: io::Error) -> JetError {
        JetError::FileNotFound(cause)
    }
}

impl From<git2::Error> for JetError {
    fn from(cause: git2::Error) -> JetError {
        JetError::NotAGitRepository(cause)
    }
}

impl From<reqwest::Error> for JetError {
    fn from(cause: reqwest::Error) -> JetError {
        JetError::JiraResourceNotFound(cause)
    }
}

impl From<ConfigAlreadyExist> for JetError {
    fn from(cause: ConfigAlreadyExist) -> JetError {
        JetError::ConfigAlreadyExist(cause)
    }
}

impl From<toml::ser::Error> for JetError {
    fn from(cause: toml::ser::Error) -> JetError {
        JetError::TomlError(cause)
    }
}

impl From<ConfigError> for JetError {
    fn from(cause: ConfigError) -> JetError {
        JetError::ConfigError(cause)
    }
}

#[derive(Debug)]
pub struct ConfigAlreadyExist {}

impl fmt::Display for ConfigAlreadyExist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path = env::current_dir().unwrap();
        write!(f, "Config already exist : {}", path.display())
    }
}

impl Error for ConfigAlreadyExist {
    fn description(&self) -> &str {
        "Cannot override existing .jetcli config"
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(&ConfigAlreadyExist {})
    }
}
