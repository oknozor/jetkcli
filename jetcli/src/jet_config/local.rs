#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub branching_model: Vec<String>,
    pub reviewers: Vec<String>,
    pub default_assignee: String,
}