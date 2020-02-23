#[derive(Serialize, Deserialize, Debug)]
pub struct Transition {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionRequest {
    pub transition: TransitionRequestInner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionRequestInner {
    pub id: String,
}

impl TransitionRequest {
    pub fn new(id: &str) -> TransitionRequest {
        TransitionRequest {
            transition: TransitionRequestInner { id: id.into() },
        }
    }
}
