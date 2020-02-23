mod jql;
mod search;

use jql::*;
use search::{Search, *};

pub mod model;

use crate::{
    error::JetError,
    jira::model::{
        assignee::AssigneeRequest,
        issue::Issue,
        response::Transitions,
        transition::TransitionRequest,
    },
};
use failure::_core::marker::PhantomData;
use model::{project::Project, response::IssueSearch, ToPage};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

const PROJECT: &str = "/rest/api/2/project";
const ISSUE: &str = "/rest/api/2/issue";
const SEARCH: &str = "/rest/api/2/search";

pub struct Jira {
    pub client: reqwest::Client,
    pub credentials: Credentials,
    pub host: String,
}

pub struct Credentials {
    pub username: String,
    pub password: String,
}

// Allow mapping from http status to JetError
struct ResponseWrapper<T> {
    response: Response,
    phantom: PhantomData<T>,
}

impl ResponseWrapper<()> {
    fn get(&mut self) -> Result<(), JetError> {
        match self.response.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(()),
            status => Err(JetError::HttpError {
                status: status.as_str().into(),
            }),
        }
    }
}
impl<T> ResponseWrapper<T>
where
    T: DeserializeOwned,
{
    fn json(&mut self) -> Result<T, JetError> {
        match self.response.status() {
            StatusCode::NO_CONTENT | StatusCode::OK => Ok(self.response.json()?),
            status => Err(JetError::HttpError {
                status: status.as_str().into(),
            }),
        }
    }

    fn from(response: Response) -> ResponseWrapper<T> {
        ResponseWrapper {
            response,
            phantom: PhantomData,
        }
    }
}

impl Credentials {
    pub fn username_simple(&self) -> String {
        let split: Vec<&str> = self.username.split('@').collect();
        let split = *split.get(0).unwrap();
        split.to_string()
    }

    fn pass(&self) -> Option<&str> {
        Some(&self.password)
    }
}

impl Jira {
    pub fn new(
        credentials: Credentials,
        host: &str,
    ) -> Jira {
        let host = host.into();

        Jira {
            client: reqwest::Client::new(),
            credentials,
            host,
        }
    }

    pub fn get_open_issues(&self) -> Result<String, JetError> {
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

        let response = self
            .client
            .post(&format!("{}/{}", self.host, SEARCH))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .json(&query)
            .send()?;

        ResponseWrapper::<IssueSearch>::from(response)
            .json()
            .map(|search_result| search_result.to_page())
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>, JetError> {
        let response = self
            .client
            .get(&format!("{}{}", self.host, PROJECT))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .send()?;

        ResponseWrapper::from(response).json()
    }

    pub fn get_project(
        &self,
        project_name: &str,
    ) -> Result<Project, JetError> {
        let response = self
            .client
            .get(&format!("{}{}/{}", self.host, PROJECT, project_name))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .send()?;

        ResponseWrapper::from(response).json()
    }

    pub fn get_issue_by_id(
        &self,
        issue_id: &str,
    ) -> Result<Issue, JetError> {
        let response = self
            .client
            .get(&format!("{}{}/{}", self.host, ISSUE, issue_id))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .send()?;

        ResponseWrapper::from(response).json()
    }

    pub fn get_transitions(
        &self,
        issue_id: &str,
    ) -> Result<Transitions, JetError> {
        let response = self
            .client
            .get(&format!("{}{}/{}/transitions", self.host, ISSUE, issue_id))
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .send()?;

        ResponseWrapper::from(response).json()
    }

    pub fn do_transition(
        &self,
        issue_id: &str,
        transition_id: &str,
    ) -> Result<(), JetError> {
        let request = TransitionRequest::new(transition_id);
        let body = serde_json::to_string(&request).unwrap();

        let response = self
            .client
            .post(&format!("{}{}/{}/transitions", self.host, ISSUE, issue_id))
            .header("Content-type", "application/json")
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .body(body)
            .send()?;

        ResponseWrapper::from(response).get()
    }

    pub fn assign(
        &self,
        issue_id: &str,
        username: &str,
    ) -> Result<(), JetError> {
        let request = AssigneeRequest::new(username);
        let body = serde_json::to_string(&request).unwrap();

        let response = self
            .client
            .put(&format!("{}{}/{}/assignee", self.host, ISSUE, issue_id))
            .header("Content-type", "application/json")
            .basic_auth(&self.credentials.username, self.credentials.pass())
            .body(body)
            .send()?;

        ResponseWrapper::from(response).get()
    }
}
