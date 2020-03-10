use crate::{command::JetJiraCommand, error::JetError, git::GitRepo, jira::Jira};

pub struct StatusCommand {
    detail: bool,
}

impl Default for StatusCommand {
    fn default() -> Self {
        StatusCommand { detail: false }
    }
}

impl JetJiraCommand for StatusCommand {
    fn execute(
        &self,
        client: &mut Jira,
    ) -> Result<(), JetError> {
        let git = GitRepo::open()?;
        let branch_name = git.get_current_branch_name()?;
        println!("On branch {}", branch_name);
        let issue_key = super::branch_name_to_issue_key(&branch_name);

        if let Some(issue_key) = issue_key {
            println!("Related commits : ");
            git.search_commits(&issue_key)?
                .iter()
                .for_each(|commit| println!("\t{}", commit));

            let result = client.get_issue_by_id(&issue_key)?;
            println!("Working on {}", issue_key);

            if let Some(status) = result.fields.status {
                println!("\tStatus : {}", status.name);
            }

            if let Some(assignee) = result.fields.assignee {
                println!("\tAssignee : {}", assignee.name);
            }

            if let Some(summary) = result.fields.summary {
                println!("\tSummary : {}", summary);
            }
        } else {
            println!("# Current branch as no matching jira issue")
        }

        Ok(())
    }
}
