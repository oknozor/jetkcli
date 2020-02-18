use std::{io, env};
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum JetError {
    ConfigNotFound(io::Error),
    ConfigAlreadyExist(ConfigAlreadyExist),
    NotAGitRepository(git2::Error),
    JiraResourceNotFound(reqwest::Error),
    Other
}

impl fmt::Display for JetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JetError::ConfigAlreadyExist(ref cause) => write!(f, "Config already exist : {}", cause),
            JetError::ConfigNotFound(ref cause) => write!(f, "Config file not found : {}", cause),
            JetError::NotAGitRepository(ref cause) => write!(f, "Current dir is not a git repository : {}", cause),
            JetError::JiraResourceNotFound(ref cause) => write!(f, "Error fetching resource from jira : {}", cause),
            JetError::Other=> write!(f, "Unknown Jet error"),
        }
    }
}

impl Error for JetError {
    fn description(&self) -> &str {
        match *self {
            JetError::ConfigNotFound(ref cause) => cause.description(),
            JetError::ConfigAlreadyExist(ref cause) => cause.description(),
            JetError::NotAGitRepository(ref cause) => cause.description(),
            JetError::JiraResourceNotFound(ref cause) => cause.description(),
            JetError::Other => "Unknown jet error!",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            JetError::ConfigNotFound(ref cause) => Some(cause),
            JetError::ConfigAlreadyExist(ref cause) => Some(cause),
            JetError::NotAGitRepository(ref cause) => Some(cause),
            JetError::JiraResourceNotFound(ref cause) => Some(cause),
            JetError::Other => None,
        }
    }
}

impl From<io::Error> for JetError {
    fn from(cause: io::Error) -> JetError {
        JetError::ConfigNotFound(cause)
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
        "Cannot override existing jet config"
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(&ConfigAlreadyExist {})
    }
}
