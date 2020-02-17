use crate::error::JetError;
use crate::jira::Jira;

pub mod checkout;
pub mod init;
pub mod log;
pub mod request;

pub trait JetCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError>;
}
