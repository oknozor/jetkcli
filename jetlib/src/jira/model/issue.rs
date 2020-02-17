use super::status::Status;
use super::ToPage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: Fields,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
    pub status: Option<Status>,
    pub summary: Option<String>,
}

impl ToPage for Issue {
    fn to_page(&self) -> String {
        format!("[{}]\n{}", self.key, self.fields.to_page())
    }
}

impl ToPage for Fields {
    fn to_page(&self) -> String {
        let status = self.status.as_ref().unwrap_or(&Status::empty()).to_page();
        let summary = self.summary.as_ref();
        let summary = summary.unwrap_or(&"".to_owned()).to_owned();
        format!("{}\nDescription : {}", status, summary)
    }
}
