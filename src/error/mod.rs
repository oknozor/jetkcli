use config::ConfigError;
use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum JetError {
    #[fail(display = "IO error")]
    IOError(#[fail(cause)] io::Error),

    #[fail(display = "Config error")]
    ConfigError(#[fail(cause)] ConfigError),

    #[fail(display = "Git error")]
    GitError(#[fail(cause)] git2::Error),

    #[fail(display = "Toml error")]
    TomlError(#[fail(cause)] toml::ser::Error),

    #[fail(display = "Request error")]
    RequestError(#[fail(cause)] reqwest::Error),

    #[fail(display = "Config file exists : {}", path)]
    ConfigAlreadyExist { path: String },

    #[fail(display = "More than one matching branch {:?}", branches)]
    MoreThanOneIssueBranch { branches: Vec<String> },
    #[fail(display = "No such branch {}", branch)]
    BranchNotFound { branch: String },
    #[fail(display = "Git index is empty")]
    EmptyIndex,

    #[fail(display = "Http error : {}", status)]
    HttpError { status: String },
}

impl From<io::Error> for JetError {
    fn from(cause: io::Error) -> JetError {
        JetError::IOError(cause)
    }
}

impl From<git2::Error> for JetError {
    fn from(cause: git2::Error) -> JetError {
        JetError::GitError(cause)
    }
}

impl From<reqwest::Error> for JetError {
    fn from(cause: reqwest::Error) -> JetError {
        JetError::RequestError(cause)
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
