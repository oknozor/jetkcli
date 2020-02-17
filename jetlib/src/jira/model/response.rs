use super::issue::Issue;
use super::ToPage;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueSearch {
    pub issues: Vec<Issue>,
}

impl ToPage for IssueSearch {
    fn to_page(&self) -> String {
        self.issues.iter().fold(String::new(), |acc, issue| {
            format!("{}\n\n{}", acc, issue.to_page())
        })
    }
}
