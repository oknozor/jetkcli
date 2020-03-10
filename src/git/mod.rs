use crate::error::JetError;
use git2::{BranchType, Commit, DiffOptions, Object, ObjectType, Repository};
use std::path::Path;

/// A Simple wrapper around `git2::Repository` providing high level method to
/// handle git repo operations.
pub(crate) struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Returns the currently open git repository
    pub fn open() -> Result<GitRepo, git2::Error> {
        let repo = Repository::discover(".")?;
        Ok(GitRepo { repo })
    }

    /// Return the working root directory of the current repository
    pub fn get_repo_dir(&self) -> Option<&Path> {
        self.repo.workdir()
    }

    /// checkout an existing git branch
    /// panics if the current directory is not in a git repository
    pub fn checkout(
        &self,
        branch_name: &str,
    ) -> Result<(), JetError> {
        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|err| err.into())
    }

    /// Checkout the first matched branch name containing  the input string
    /// ## Example
    ///
    /// Let's say we are on branch `develop` and want to checkout `feat/JET-1`
    /// ```rust, no_run
    /// let repo = GitRepo::open();
    /// repo.find_checkout("JET-1"); // match `feat/JET-1` and checkout
    /// ```
    pub fn find_checkout(
        &self,
        issue_key: &str,
    ) -> Result<(), JetError> {
        let branches = self.repo.branches(Some(BranchType::Local))?;
        let matches: Vec<String> = branches
            .map(|branch| {
                let branch = branch.unwrap().0;
                branch.name().unwrap().unwrap().to_owned()
            })
            .filter(|branch_name| branch_name.contains(issue_key))
            .collect();

        if matches.is_empty() {
            Err(JetError::BranchNotFound {
                branch: issue_key.into(),
            })
        } else if matches.len() == 1 {
            self.checkout(&matches[0])
        } else {
            Err(JetError::MoreThanOneIssueBranch { branches: matches })
        }
    }

    /// Given an input string, return a list string representing the matching
    /// commit messages.
    pub fn search_commits(
        &self,
        term: &str,
    ) -> Result<Vec<String>, JetError> {
        let repo = &self.repo;
        let mut result = vec![];
        if let Some(oid) = repo.head()?.target() {
            let commit = repo.find_commit(oid)?;
            let matching_commit = &commit.message().filter(|message| message.contains(term));
            if let Some(_matching_commit) = matching_commit {
                result.push(self.commit_to_string(&commit));
                //TODO
            }
        }
        Ok(result)
    }

    /// Pretty print commits, think of it as `git log`
    fn commit_to_string(
        &self,
        commit: &Commit,
    ) -> String {
        let mut output = String::new();
        output.push_str(&commit.id().to_string()[0..8]);

        if commit.parents().len() > 1 {
            output.push_str("Merge:");
            for id in commit.parent_ids() {
                output.push_str("Merge:");
                output.push_str(&format!(" {:.8}", id));
            }
            output.push('\n');
        }

        // print_time(&author.when(), "Date:   ");
        println!();

        for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
            output.push_str(&format!(" - {}", line));
        }

        let author = commit.author();
        let author_name = author.name();

        if let Some(name) = author_name {
            output.push_str(&format!(" - {}", name));
        }

        output.push('\n');
        output
    }

    /// Create a new branch with the given `branch_name` and perform a checkout.
    /// Just like `git checkout -b {branch_name}`
    pub fn create_and_checkout(
        &self,
        branch_name: &str,
    ) -> Result<(), JetError> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        let head = revwalk.last().expect("No HEAD in revwalk");
        let head = head?;
        let head = self.repo.find_commit(head)?;

        let _ = &self.repo.branch(branch_name, &head, false)?;
        self.checkout(branch_name)
    }

    /// Create a new commit with the given commit message
    pub fn commit(
        &self,
        message: String,
    ) -> Result<(), JetError> {
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
