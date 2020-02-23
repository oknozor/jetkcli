use std::collections::HashMap;

use moins::{Color, Moins, PagerOptions};

use crate::{command::JetJiraCommand, error::JetError, jira::Jira};

pub struct ListIssuesCommand;

impl JetJiraCommand for ListIssuesCommand {
    fn execute(
        &self,
        client: &mut Jira,
    ) -> Result<(), JetError> {
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
        Moins::run(issues, Some(options));
        Ok(())
    }
}
