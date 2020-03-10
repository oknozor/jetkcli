use crate::{command::JetCommand, error::JetError, git::GitRepo};

pub struct CommitCommand {
    pub prefix: String,
    pub message: String,
    pub scope: Option<String>,
}

impl JetCommand for CommitCommand {
    fn execute(&self) -> Result<(), JetError> {
        let git_repo = GitRepo::open()?;
        let branch = git_repo.get_current_branch_name()?;

        let issue = super::branch_name_to_issue_key(&branch);

        let message = match (&self.scope, issue.as_ref()) {
            (Some(scope), Some(issue)) => format!(
                "{prefix}({scope}): {message} [{issue}]",
                prefix = &self.prefix,
                scope = scope,
                message = &self.message,
                issue = issue
            ),
            (None, Some(issue)) => format!(
                "{prefix}: {message} [{issue}]",
                prefix = &self.prefix,
                message = &self.message,
                issue = issue
            ),
            (Some(scope), None) => format!(
                "{prefix}({scope}): {message}g",
                prefix = &self.prefix,
                scope = scope,
                message = &self.message,
            ),
            (None, None) => format!(
                "{prefix}: {message}",
                prefix = &self.prefix,
                message = &self.message,
            ),
        };

        git_repo.commit(message)
    }
}
