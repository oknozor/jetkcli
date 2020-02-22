use crate::command::JetJiraCommand;
use crate::error::JetError;
use crate::git::GitRepo;
use crate::jira::Jira;
use crate::settings::PROJECT_SETTINGS_SHARED;

pub struct CheckoutCommand {
    pub target_issue: String,
    pub prefix: String,
    pub new_branch: bool,
}

impl JetJiraCommand for CheckoutCommand {
    fn execute(&self, jira: &mut Jira) -> Result<(), JetError> {
        let issue = jira.get_issue_by_id(&self.target_issue)?;
        let git = GitRepo::open()?;
        let separator = &PROJECT_SETTINGS_SHARED.git.branch_separator;
        let branch_name = format!("{}{}{}", &self.prefix, separator, issue.key);

        if self.new_branch {
            git.create_and_checkout(&branch_name)?;
            println!(
                "Checkout new branch, currently working on jira issue {}",
                issue.key
            );
        } else {
            git.checkout(&branch_name)?;
            println!(
                "Checkout branch, currently working on jira issue {}",
                issue.key
            );
        }

        Ok(())
    }
}
