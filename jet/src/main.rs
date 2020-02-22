extern crate clap;

use clap::{App, Arg, SubCommand};

use jetlib::command::checkout::CheckoutCommand;
use jetlib::command::commit::CommitCommand;
use jetlib::command::info::InfoCommand;
use jetlib::command::init::InitCommand;
use jetlib::command::issues::ListIssuesCommand;
use jetlib::command::{JetCommand, JetJiraCommand};
use jetlib::jira::Jira;
use jetlib::settings::shared::ProjectSettingsShared;
use jetlib::settings::GLOBAL_SETTINGS;
use jetlib::settings::PROJECT_SETTINGS_SHARED;
use std::borrow::BorrowMut;

fn main() {
    // Generate pre-formatted commit commands
    let commit_types = if let Ok(settings) = ProjectSettingsShared::get() {
        Some(
            settings
                .git
                .commit_types
                .iter()
                .map(|prefix| {
                    SubCommand::with_name(&prefix)
                        .help("Create a pre-formatted according to your jet config file")
                        .arg(Arg::with_name("message").help("The commit message"))
                        .arg(Arg::with_name("scope").help("The scope of th e commit message"))
                })
                .collect(),
        )
    } else {
        None
    };

    let checkouts = if let Ok(settings) = ProjectSettingsShared::get() {
        Some(
            settings
                .git
                .branch_types
                .iter()
                .map(|prefix| SubCommand::with_name(&prefix).arg(Arg::with_name("ISSUE")))
                .collect::<Vec<App>>(),
        )
    } else {
        None
    };

    let matches = App::new("Jet")
        .version("0.1")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("Jira kung fu client")
        .subcommands(commit_types.unwrap_or_else(|| vec![SubCommand::with_name("commit")]))
        .subcommand(
            SubCommand::with_name("checkout")
                .arg(
                    Arg::with_name("branch")
                        .long("branch")
                        .short("b")
                        .help("template issue branch"),
                )
                .subcommands(checkouts.unwrap_or_else(|| vec![SubCommand::with_name("checkout")])),
        )
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
                        .required(false),
                )
                .about("init")
                .help("Init a .jetcli project inside a git repository"),
        )
        .subcommand(SubCommand::with_name("issues").about("display all ongoing issues"))
        .subcommand(SubCommand::with_name("info").about("dump info on the current jet project"))
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            "init" => {
                let init = matches.subcommand_matches("init").unwrap();
                let project_name = init.value_of("project").unwrap_or("unwraped");
                let server_name = init.value_of("server");

                InitCommand::new(project_name, server_name)
                    .execute()
                    .expect("Error during project initialisation");
            }
            "info" => InfoCommand
                .execute()
                .expect("Error during fetching project info"),
            "issues" => {
                // We need the http client
                let host = &PROJECT_SETTINGS_SHARED.jira.server_url;
                let credentials = GLOBAL_SETTINGS.current_credentials();
                let mut jira = Jira::new(credentials, host);

                ListIssuesCommand
                    .execute(jira.borrow_mut())
                    .expect("Error while fetching jira issues");
            }
            "checkout" => {
                let checkout = matches
                    .subcommand_matches("checkout")
                    .expect("Unable to get checkout subcommands");
                let settings = ProjectSettingsShared::get().expect("Unable to get shared settings");
                let new_branch = checkout.is_present("branch");
                settings.git.branch_types.iter().for_each(|prefix| {
                    if let Some(args) = checkout.subcommand_matches(&prefix) {
                        let prefix = prefix.to_owned();

                        // FIXME
                        let target_issue = args.value_of("ISSUE");
                        println!("{:?}", target_issue);

                        let target_issue = target_issue
                            .expect("Expected an issue key as argument")
                            .into();

                        let command = CheckoutCommand {
                            target_issue,
                            prefix,
                            new_branch,
                        };

                        let host = &PROJECT_SETTINGS_SHARED.jira.server_url;
                        let credentials = GLOBAL_SETTINGS.current_credentials();
                        let mut jira = Jira::new(credentials, host);

                        command
                            .execute(&mut jira)
                            .expect("Error during checkout command");
                    };
                });
            }
            _other => {
                let settings = ProjectSettingsShared::get().expect("Unable to get shared settings");
                settings.git.commit_types.iter().for_each(|prefix| {
                    if let Some(args) = matches.subcommand_matches(&prefix) {
                        let message = args.value_of("message").unwrap().to_string();
                        let scope = args.value_of("scope").map(|scope| scope.to_string());
                        let prefix = prefix.to_owned();

                        let commit_command = CommitCommand {
                            message,
                            scope,
                            prefix,
                        };

                        commit_command.execute().unwrap();
                    }
                })
            }
        }
    };
}
