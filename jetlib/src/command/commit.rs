use crate::command::JetCommand;
use crate::error::JetError;
use git2::Repository;

pub struct CommitCommand {
    pub prefix: String,
    pub message: String,
    pub scope: String,
}

impl JetCommand for CommitCommand {
    fn execute(&self) -> Result<(), JetError> {
        let repo = Repository::open("./")?;
        let sig = repo.signature()?;
        let tree_id = repo.index()?.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let head_target = repo.head().unwrap().target().unwrap(); // TODO : Handle None
        let tip = repo.find_commit(head_target)?;

        let message = &format!(
            "{prefix}({scope}): {message} ({issue}",
            prefix = &self.prefix,
            scope = &self.scope,
            message = &self.message,
            issue = "DUM-01"
        );

        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&tip])
            .map(|_| ())
            .map_err(|err| JetError::from(err))
    }
}
