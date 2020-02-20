use crate::command::JetCommand;
use crate::error::JetError;
use crate::git::GitRepo;

pub struct CommitCommand {
    pub prefix: String,
    pub message: String,
    pub scope: Option<String>,
}

impl JetCommand for CommitCommand {
    fn execute(&self) -> Result<(), JetError> {
        let message = if let Some(scope) = &self.scope {
            format!(
                "{prefix}({scope}): {message} ({issue})",
                prefix = &self.prefix,
                scope = scope,
                message = &self.message,
                issue = "DUM-01"
            )
        } else {
            format!(
                "{prefix}: {message} ({issue})",
                prefix = &self.prefix,
                message = &self.message,
                issue = "DUM-01"
            )
        };
        let git_repo = GitRepo::open()?;
        git_repo.commit(message)
    }
}
