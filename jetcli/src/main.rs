extern crate clap;

use clap::{App, Arg, SubCommand};

use jet::command::commit::CommitCommand;
use jet::command::init::InitCommand;
use jet::command::issues::ListIssuesCommand;
use jet::command::{JetCommand, JetJiraCommand};
use jet::jira::Credentials;
use jet::jira::Jira;
use jet::settings::global::GlobalSettings;
use jet::settings::local::ProjectSettings;

fn main() {
    let settings = GlobalSettings::new().unwrap();

    let server = settings
        .servers
        .get("ineat")
        .expect(&format!("server {} does not exist in your config", "gl"));

    let credentials = Credentials {
        username: server.username.to_owned(),
        password: server.password.to_owned(),
    };

    // Generate pre-formatted commit commands
    let commit_types = if let Ok(settings) = ProjectSettings::new() {
        Some(
            settings
                .commit_types
                .iter()
                .map(|prefix| SubCommand::with_name(&prefix))
                .collect(),
        )
    } else {
        None
    };

    let mut jira = Jira::new(credentials, &server.url);

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
                        .help("remote server name in the global .jet config file")
                        .required(true),
                )
                .about("init")
                .help("Init a .jet project inside a git repository"),
        )
        .subcommand(SubCommand::with_name("issues").about("display all ongoing issues"))
        .get_matches();

    let _ = ProjectSettings::new().unwrap();

    if let Ok(settings) = ProjectSettings::new() {
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
            ListIssuesCommand.execute(&mut jira).unwrap();
        } else if let Some(init) = matches.subcommand_matches("init") {
            let project_name = init.value_of("project").unwrap();
            let server_name = init.value_of("server").unwrap();
            InitCommand::new(project_name, server_name)
                .execute(&mut jira)
                .unwrap();
        }
    }
}
