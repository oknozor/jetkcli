#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub jql: String,
    pub start_at: i32,
    pub max_results: i32,
    pub fields: Vec<Field>,
}

//https://confluence.atlassian.com/jiracorecloud/advanced-searching-fields-reference-765593716.html
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Field {
    Summary,
    Status,
    Description,
}

impl Field {
    pub fn all() -> Vec<Field> {
        vec![Field::Summary, Field::Status, Field::Description]
    }
}
