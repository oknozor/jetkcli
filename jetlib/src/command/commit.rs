use crate::command::JetCommand;
use crate::error::JetError;
use git2::{Repository, DiffOptions, ObjectType, Object};

pub struct CommitCommand {
    pub prefix: String,
    pub message: String,
    pub scope: Option<String>,
}

impl JetCommand for CommitCommand {
    fn execute(&self) -> Result<(), JetError> {
        let repo = Repository::open("./")?;


        let sig = repo.signature()?;
        let tree_id = repo.index()?.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let head = tree_to_treeish(&repo, Some(&"HEAD".to_string()))?.unwrap();
        let mut options = DiffOptions::new();


        let deltas = repo
            .diff_tree_to_index(head.as_tree(), None, Some(&mut options))
            .unwrap()
            .deltas()
            .len();

        if deltas == 0 {
            return Err(JetError::EmptyIndex);
        }

        let head_target = repo.head().unwrap().target().unwrap(); // TODO : Handle None
        let tip = repo.find_commit(head_target)?;

        let message = if let Some(scope) = &self.scope {
            format!(
                "{prefix}({scope}): {message} ({issue}",
                prefix = &self.prefix,
                scope = scope,
                message = &self.message,
                issue = "DUM-01"
            )
        } else {
            format!(
                "{prefix}: {message} ({issue}",
                prefix = &self.prefix,
                message = &self.message,
                issue = "DUM-01"
            )
        };

        repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&tip])
            .map(|_| ())
            .map_err(JetError::from)
    }
}


fn tree_to_treeish<'a>(
    repo: &'a Repository,
    arg: Option<&String>,
) -> Result<Option<Object<'a>>, git2::Error> {
    let arg = match arg {
        Some(s) => s,
        None => return Ok(None),
    };
    let obj = repo.revparse_single(arg)?;
    let tree = obj.peel(ObjectType::Tree)?;
    Ok(Some(tree))
}
