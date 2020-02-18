use crate::command::JetCommand;
use crate::jira::Jira;
use crate::error::JetError;

pub struct CheckoutCommand;

impl JetCommand for CheckoutCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError> {
        unimplemented!()
    }
}