use super::ToPage;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status_category: StatusCategory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusCategory {
    pub id: i32,
    pub key: String,
    pub name: String,
}

impl Status {
    pub fn empty() -> Status {
        Status {
            id: "".to_owned(),
            name: "".to_owned(),
            description: "".to_owned(),
            status_category: StatusCategory::empty(),
        }
    }
}

impl ToPage for Status {
    fn to_page(&self) -> String {
        format!(
            "Status : {}\n{}",
            self.name,
            self.status_category.to_page()
        )
    }
}

impl StatusCategory {
    fn empty() -> StatusCategory {
        StatusCategory {
            id: 0,
            key: "".to_owned(),
            name: "".to_owned(),
        }
    }
}

impl ToPage for StatusCategory {
    fn to_page(&self) -> String {
        format!(
            "Status description : {}",
            self.name,
        )
    }
}

