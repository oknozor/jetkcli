extern crate clap;

use clap::{App, Arg, SubCommand};

use jetlib::command::commit::CommitCommand;
use jetlib::command::init::InitCommand;
use jetlib::command::issues::ListIssuesCommand;
use jetlib::command::{JetCommand, JetJiraCommand};
use jetlib::jira::Jira;
use jetlib::settings::local::ProjectSettingsShared;
use jetlib::settings::PROJECT_SETTINGS_SHARED;
use jetlib::settings::GLOBAL_SETTINGS;
use std::borrow::BorrowMut;

fn main() {
    // Generate pre-formatted commit commands
    let commit_types = if let Ok(settings) = ProjectSettingsShared::get() {
        Some(
            settings
                .commit_types
                .iter()
                .map(|prefix|
                    SubCommand::with_name(&prefix)
                        .help("Create a pre-formatted according to your jet config file")
                        .arg(Arg::with_name("message")
                            .help("The commit message"))
                        .arg(Arg::with_name("scope")
                            .help("The scope of th e commit message")))
                .collect()
        )
    } else {
        None
    };


    let matches = App::new("Jet")
        .version("0.1")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("Jira kung fu client")
        .subcommands(commit_types.unwrap_or(vec![SubCommand::with_name("commit")]))
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("project")
                        .long("project")
                        .short("p")
                        .takes_value(true)
                        .help("Project name in Jira")
                        .required(true),
                )
                .arg(
                    Arg::with_name("server")
                        .long("server")
                        .short("s")
                        .takes_value(true)
                        .help("remote server name in the global .jetcli config file")
                        .required(true),
                )
                .about("init")
                .help("Init a .jetcli project inside a git repository"),
        )
        .subcommand(SubCommand::with_name("issues").about("display all ongoing issues"))
        .get_matches();


    if let Ok(settings) = ProjectSettingsShared::get() {
        settings.commit_types.iter().for_each(|prefix| {
            if let Some(_arg) = matches.subcommand_matches(&prefix) {

                let commit_command = CommitCommand {
                    prefix: "placeholder".to_string(),
                    message: "placeholder".to_string(),
                    scope: "placeholder".to_string(),
                };

                commit_command.execute().unwrap();
            }
        })
    } else {


        if let Some(_matches) = matches.subcommand_matches("issues") {

            // We need the http client
            let host = &PROJECT_SETTINGS_SHARED.server_url;
            let credentials = GLOBAL_SETTINGS.current_credentials();
            let mut  jira = Jira::new(credentials, host);

            ListIssuesCommand.execute(jira.borrow_mut()).unwrap();
        } else if let Some(init) = matches.subcommand_matches("init") {

            let project_name = init.value_of("project").unwrap_or("unwraped");
            let server_name = init.value_of("server").unwrap_or("unwraped");

            InitCommand::new(project_name, server_name)
                .execute()
                .unwrap();

        }
    }
}
