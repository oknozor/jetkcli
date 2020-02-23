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

    pub fn checkout(&self, branch_name: &str) -> Result<(), JetError> {
        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|err| err.into())
    }

    pub fn create_and_checkout(&self, branch_name: &str) -> Result<(), JetError> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        let head = revwalk.last().expect("No HEAD in revwalk");
        let head = head?;
        let head = self.repo.find_commit(head)?;

        let _ = &self.repo.branch(branch_name, &head, false)?;
        self.checkout(branch_name)
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
            let head_target = head.target().expect("Cannot get HEAD target");
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
            });

            Err(JetError::EmptyIndex)
        }
    }
    pub fn get_current_branch_name(&self) -> Result<String, JetError> {
        let head = &self.repo.head()?;
        let head = head.shorthand();
        let branch_name = head.expect("Cannot get HEAT").into();
        Ok(branch_name)
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
