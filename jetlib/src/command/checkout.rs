use crate::command::JetJiraCommand;
use crate::error::JetError;
use crate::jira::Jira;

pub struct CheckoutCommand;

impl JetJiraCommand for CheckoutCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError> {
        unimplemented!()
    }
}
