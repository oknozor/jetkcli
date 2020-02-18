use moins::Color;
use moins::Moins;
use moins::PagerOptions;
use std::collections::HashMap;
use crate::command::JetCommand;
use crate::jira::Jira;
use crate::error::JetError;


pub struct LogCommand;

impl JetCommand for LogCommand {
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