use crate::{
    command::{JetCommand, JetJiraCommand},
    error::JetError,
    git::GitRepo,
    jira::Jira,
    settings::{internal::InternalSettings, GLOBAL_SETTINGS, PROJECT_SETTINGS_SHARED},
};
use reqwest::StatusCode;
use std::{fs::OpenOptions, io::Write, process};

pub struct CheckoutCommand {
    pub target_issue: String,
    pub prefix: String,
    pub new_branch: bool,
}

pub struct SimpleCheckoutCommand {
    pub target_issue: String,
}

impl JetJiraCommand for CheckoutCommand {
    fn execute(
        &self,
        jira: &mut Jira,
    ) -> Result<(), JetError> {
        let issue = jira.get_issue_by_id(&self.target_issue)?;

        let git = GitRepo::open()?;
        let separator = &PROJECT_SETTINGS_SHARED.git.branch_separator;
        let branch_name = format!("{}{}{}", &self.prefix, separator, issue.key);

        if self.new_branch {
            // Transition jira issue state to WIP
            let target_state = &PROJECT_SETTINGS_SHARED.jira.workflow.wip;

            let internal_settings = InternalSettings::get()?;

            let transition_id =
                if let Some(transition_id) = internal_settings.workflow.get(target_state) {
                    // Transition id is cached in the internal config file
                    transition_id.clone()
                } else {
                    // Get transition id from jira API
                    let response = jira.get_transitions(issue.key.as_str())?;
                    let in_progress = response
                        .transitions
                        .iter()
                        .find(|transition| &transition.name == target_state)
                        .expect("unable to find in progress transition");

                    // Write transition id to internal config
                    let internal_settings =
                        InternalSettings::add_workflow(&in_progress.name, &in_progress.id)?;

                    let mut path = git.get_repo_dir().unwrap().to_path_buf();
                    path.push(".jet/config.internal.toml");

                    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

                    file.write_all(internal_settings.to_string().as_bytes())
                        .map_err(JetError::from)?;

                    in_progress.id.clone()
                };

            match jira.do_transition(&issue.key, &transition_id)?.status() {
                StatusCode::NO_CONTENT => (),
                err_status => {
                    eprintln!(
                        "Could not transition Jira issue status code {}",
                        err_status.as_u16()
                    );
                    process::exit(1);
                }
            };

            let username = GLOBAL_SETTINGS.current_credentials().username_simple();
            match jira.assign(&issue.key, &username)?.status() {
                StatusCode::NO_CONTENT => (),
                err_status => {
                    eprintln!(
                        "Could not assign Jira issue to {} status code {}",
                        username,
                        err_status.as_u16()
                    );
                    process::exit(1);
                }
            };

            // assign issue
            git.create_and_checkout(&branch_name)?;
            println!(
                "Checkout new branch, currently working on jira issue {}",
                issue.key
            );
        } else {
            git.checkout(&branch_name)?;
            println!(
                "Checkout branch, currently working on jira issue {}",
                issue.key
            );
        }

        Ok(())
    }
}

impl SimpleCheckoutCommand {
    pub fn new(target_issue: &str) -> SimpleCheckoutCommand {
        SimpleCheckoutCommand {
            target_issue: target_issue.into(),
        }
    }
}

impl JetCommand for SimpleCheckoutCommand {
    fn execute(&self) -> Result<(), JetError> {
        let git = GitRepo::open()?;
        git.find_checkout(&self.target_issue)
            .map_err(JetError::from)
    }
}
