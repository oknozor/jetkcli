mod jql;
mod search;

use jql::*;
use reqwest;
use search::Search;
use search::*;

pub mod model;

use model::response::IssueSearch;
use model::ToPage;
use model::project::Project;

const PROJECT: &str = "/rest/api/2/project";
const SEARCH: &str = "/rest/api/2/search";

pub struct Jira<'a> {
    pub client: reqwest::Client,
    pub credentials: Credentials,
    pub host: &'a str,
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    fn username_simple(&self) -> String {
        let split: Vec<&str> = self.username.split('@').collect();
        split.get(0).unwrap().to_string()
    }

    fn pass(&self) -> Option<&str> {
        Some(&self.password)
    }
}

impl<'a> Jira<'a> {
    pub fn new(credentials: Credentials, host: &str) -> Jira {
        Jira {
            client: reqwest::Client::new(),
            credentials,
            host,
        }
    }
    pub fn get_open_issues(&self) -> Result<String, reqwest::Error> {
        let jql = jql::Query {
            terms: vec![],
            final_term: jql::Pair {
                key: Key::Assignee,
                value: self.credentials.username_simple(),
                operator: Operator::Eq,
            },
        }
            .to_string();

        let query = Search {
            jql,
            start_at: 0,
            max_results: 15,
            fields: Field::all(),
        };

        let response: Result<IssueSearch, reqwest::Error> = self
            .client
            .post(&format!("https://{}/{}", self.host, SEARCH))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .json(&query)
            .send()?
            .json();

        response.map(|response| response.to_page())
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>, reqwest::Error> {
        self
            .client
            .get(&format!("https://{}{}", self.host, PROJECT))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .send()?
            .json()
    }

    pub fn get_project(&self, project_name: &str) -> Result<Project, reqwest::Error> {
        self
            .client
            .get(&format!("https://{}{}/{}", self.host, PROJECT, project_name))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .send()?
            .json()
    }
}