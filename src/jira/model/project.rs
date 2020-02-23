#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
}
