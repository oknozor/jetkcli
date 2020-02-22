use crate::{command::JetJiraCommand, error::JetError, jira::Jira};

pub struct PushCommand;

impl JetJiraCommand for PushCommand {
    fn execute(&self, _: &mut Jira) -> Result<(), JetError> {
        unimplemented!()
    }
}
