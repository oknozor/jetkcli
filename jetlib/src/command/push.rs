use crate::command::JetJiraCommand;
use crate::error::JetError;
use crate::jira::Jira;

pub struct PushCommand;

impl JetJiraCommand for PushCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError> {
        unimplemented!()
    }
}
