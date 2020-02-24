extern crate clap;

use clap::{App, Arg, SubCommand};

use jetkcli::{
    command::{
        checkout::{CheckoutCommand, SimpleCheckoutCommand},
        commit::CommitCommand,
        info::InfoCommand,
        init::InitCommand,
        issues::ListIssuesCommand,
        JetCommand,
        JetJiraCommand,
    },
    jira::Jira,
    settings::{shared::ProjectSettingsShared, GLOBAL_SETTINGS, PROJECT_SETTINGS_SHARED},
};
use std::borrow::BorrowMut;
use jetkcli::command::status::StatusCommand;

fn main() {
    // Generate pre-formatted commit commands
    let commit_subcommands = if let Ok(settings) = ProjectSettingsShared::get() {
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

    let checkouts_subcommands = if let Ok(settings) = ProjectSettingsShared::get() {
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
        .subcommands(commit_subcommands.unwrap_or_else(|| vec![]))
        .subcommand(
            SubCommand::with_name("checkout")
                .arg(Arg::with_name("ISSUE").required(false))
                .arg(
                    Arg::with_name("branch")
                        .long("branch")
                        .short("b")
                        .help("template issue branch"),
                )
                .subcommands(checkouts_subcommands.unwrap_or_else(|| vec![])),
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
                        .help("remote server name in the global jet config file")
                        .required(false),
                )
                .about("init")
                .help("Init a jet project inside a git repository"),
        )
        .subcommand(SubCommand::with_name("status").about("Like git status but with information on the related Jira issue"))
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
            "status" => {
                let host = &PROJECT_SETTINGS_SHARED.jira.server_url;
                let credentials = GLOBAL_SETTINGS.current_credentials();
                let mut jira = Jira::new(credentials, host);

                StatusCommand::default()
                    .execute(&mut jira)
                    .unwrap();
            }
            "checkout" => {
                let checkout = matches
                    .subcommand_matches("checkout")
                    .unwrap();

                let settings = ProjectSettingsShared::get().expect("Unable to get shared settings");

                let host = &settings.jira.server_url;
                let credentials = GLOBAL_SETTINGS.current_credentials();
                let mut jira = Jira::new(credentials, host);

                let new_branch = checkout.is_present("branch");
                if let Some(issue) = checkout.value_of("ISSUE") {
                    SimpleCheckoutCommand::new(issue)
                        .execute(&mut jira)
                        .unwrap();
                } else {
                    settings.git.branch_types.iter().for_each(|prefix| {
                        if let Some(args) = checkout.subcommand_matches(&prefix) {
                            let prefix = prefix.to_owned();

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

                            command
                                .execute(&mut jira)
                                .expect("Error during checkout command");
                        };
                    });
                }
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
