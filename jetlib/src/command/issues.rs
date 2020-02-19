use std::collections::HashMap;

use moins::Color;
use moins::Moins;
use moins::PagerOptions;

use crate::command::JetJiraCommand;
use crate::error::JetError;
use crate::jira::Jira;

pub struct ListIssuesCommand;

impl JetJiraCommand for ListIssuesCommand {
    fn execute(&self, client: &mut Jira) -> Result<(), JetError> {
        let mut colors = HashMap::new();
        colors.insert("Description :".to_owned(), Color::LightBlue);
        colors.insert("Status :".to_owned(), Color::LightRed);
        colors.insert("Status description :".to_owned(), Color::LightGreen);

        let options = PagerOptions {
            colors,
            search: false,
            line_number: false,
        };

        let issues = &mut client.get_open_issues()?;
        Ok(Moins::run(issues, Some(options)))
    }
}
