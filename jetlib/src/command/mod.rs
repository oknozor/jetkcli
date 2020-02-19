use crate::error::JetError;
use crate::jira::Jira;

pub mod checkout;
pub mod commit;
pub mod init;
pub mod issues;
pub mod push;

pub trait JetCommand {
    fn execute(&self) -> Result<(), JetError>;
}

pub trait JetJiraCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError>;
}
