use crate::{
    command::JetJiraCommand,
    error::JetError,
    git::GitRepo,
    jira::Jira,
    settings::PROJECT_SETTINGS_SHARED,
};

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
            // Transition jira issue state to WIP
            let target_state = &PROJECT_SETTINGS_SHARED.jira.workflow.wip;
            let response = jira.get_transitions(issue.key.as_str())?;
            let in_progress = response
                .transitions
                .iter()
                .find(|transition| &transition.name == target_state)
                .expect("unable to find in progress transition");

            // TODO : fail if status is not 204
            let _ = jira.do_transition(&issue.key, &in_progress.id)?.status();

            // assign issue
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
