use crate::error::JetError;
use git2::{DiffOptions, Object, ObjectType, Repository};
use std::path::Path;

pub(crate) struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn open() -> Result<GitRepo, git2::Error> {
        let repo = Repository::discover(".")?;

        Ok(GitRepo { repo })
    }

    pub fn get_repo_dir(&self) -> Option<&Path> {
        self.repo.workdir()
    }

    pub fn commit(&self, message: String) -> Result<(), JetError> {
        let repo = &self.repo;
        let sig = &self.repo.signature()?;
        let tree_id = &self.repo.index()?.write_tree()?;
        let tree = &self.repo.find_tree(*tree_id)?;
        let repo_is_empty = self.repo.is_empty()?;
        let mut options = DiffOptions::new();

        let diff = if let Some(head) = &self.get_head() {
            repo.diff_tree_to_index(head.as_tree(), None, Some(&mut options))
        } else {
            repo.diff_tree_to_workdir_with_index(None, Some(&mut options))
        };

        let repo_has_deltas = if let Ok(diff) = diff {
            let deltas = diff.deltas();
            deltas.len() != 0
        } else {
            false
        };

        if !repo_is_empty && repo_has_deltas {
            let head = &self.repo.head()?;
            let head_target = head.target().unwrap();
            let tip = &self.repo.find_commit(head_target)?;

            self.repo
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&tip])
                .map(|_| ())
                .map_err(JetError::from)
        } else if repo_is_empty && repo_has_deltas {
            // First repo commit
            self.repo
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])
                .map(|_| ())
                .map_err(JetError::from)
        } else {
            let statuses = repo.statuses(None)?;
            statuses.iter().for_each(|status| {
                eprintln!("{} : {:?}", status.path().unwrap(), status.status());
                // TODO map status to string
            });

            Err(JetError::EmptyIndex)
        }
    }

    fn get_head(&self) -> Option<Object> {
        if let Ok(head) = GitRepo::tree_to_treeish(&self.repo, Some(&"HEAD".to_string())) {
            head
        } else {
            None
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
}
