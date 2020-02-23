use crate::{error::JetError, jira::Jira, settings::PROJECT_SETTINGS_SHARED};

pub mod checkout;
pub mod commit;
pub mod info;
pub mod init;
pub mod issues;
pub mod push;

pub trait JetCommand {
    fn execute(&self) -> Result<(), JetError>;
}

pub trait JetJiraCommand {
    fn execute(
        &self,
        client: &mut Jira,
    ) -> Result<(), JetError>;
}

fn branch_name_to_issue_key(branch_name: &str) -> Option<String> {
    let separator = &PROJECT_SETTINGS_SHARED.git.branch_separator;
    let split: Vec<&str> = branch_name.split(separator).collect();

    if split.len() > 1 {
        Some(split[1].into())
    } else {
        None
    }
}
