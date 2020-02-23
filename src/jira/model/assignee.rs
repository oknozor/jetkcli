#[derive(Serialize, Deserialize, Debug)]
pub struct AssigneeRequest {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignee {
    pub name: String,
}

impl AssigneeRequest {
    pub fn new(username: &str) -> AssigneeRequest {
        AssigneeRequest {
            name: username.into(),
        }
    }
}
