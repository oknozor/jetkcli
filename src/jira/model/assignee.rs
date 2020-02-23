#[derive(Serialize, Deserialize, Debug)]
pub struct AssigneeRequest {
    name: String,
}

impl AssigneeRequest {
    pub fn new(username: &str) -> AssigneeRequest {
        AssigneeRequest {
            name: username.into(),
        }
    }
}
