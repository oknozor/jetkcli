#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate clap;

mod jet_config;

use jet_config::global::GlobalSettings;
use clap::{App, SubCommand, Arg};
use jet::jira::Jira;
use jet::jira::Credentials;
use jet::command;
use jet::command::init::InitCommand;
use jet::command::JetCommand;

fn main() {
    let settings = GlobalSettings::new().expect("Unable to find jetlib config file");

    let server = settings
        .servers
        .get("ineat")
        .expect(&format!("server {} does not exist in your config", "gl"));

    let credentials = Credentials {
        username: server.username.to_owned(),
        password: server.password.to_owned(),
    };

    let mut jira = Jira::new(credentials, &server.url);

    let matches = App::new("Jet")
        .version("0.1")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("Jira kung fu client")
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("project")
                        .long("project")
                        .short("p")
                        .takes_value(true)
                        .help("Project name in Jira")
                        .required(true)
                )
                .arg(
                    Arg::with_name("server")
                        .long("server")
                        .short("s")
                        .takes_value(true)
                        .help("remote server name in the global jet config file")
                        .required(true)
                )
                .about("init")
                .help("Init a jet project inside a git repository")
        )
        .subcommand(SubCommand::with_name("issues").about("display all ongoing issues"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("issues") {
        let issues = &mut jira.get_open_issues();
        command::log::run(issues);
    } else if let Some(init) = matches.subcommand_matches("init") {
        let project_name = init.value_of("project").unwrap();
        let server_name = init.value_of("server").unwrap();
        InitCommand::new(project_name, server_name)
            .execute(&mut jira).unwrap();
    }
}
