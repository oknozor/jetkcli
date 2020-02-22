use super::issue::Issue;
use super::ToPage;
use crate::jira::model::transition::Transition;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueSearch {
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transitions {
    pub transitions: Vec<Transition>,
}

impl ToPage for IssueSearch {
    fn to_page(&self) -> String {
        self.issues.iter().fold(String::new(), |acc, issue| {
            format!("{}\n\n{}", acc, issue.to_page())
        })
    }
}
