#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenv;
use dunce;
use glob;

pub mod db;
mod grm;
pub mod models;
pub mod schema;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let cmd = App::new("Git Repository Manager")
        .version("0.1.0")
        .author("Julian Didier (theredfish)")
        .about("Git Repository Manager (GRM) helps manage GIT repositories from your terminal.")
        .subcommand(
            SubCommand::with_name("add")
                .about(
                    "Search and add your git repositories for a given path; default path is the \
                     current directory",
                ).arg(
                    Arg::with_name("path")
                        .value_name("PATH")
                        .help("A relative / absolute path or a directory hierarchy.")
                        .takes_value(true),
                ),
        ).subcommand(
            SubCommand::with_name("list")
                .about("List your saved repositories for the given pattern")
                .arg(
                    Arg::with_name("pattern")
                        .value_name("PATTERN")
                        .help("The pattern to apply")
                        .takes_value(true),
                ),
        ).subcommand(
            SubCommand::with_name("goto")
                .about("Go to the repository directory")
                .arg(
                    Arg::with_name("repo_name")
                        .value_name("REPOSITORY_NAME")
                        .help("The repository name")
                        .takes_value(true),
                ),
        ).subcommand(
            SubCommand::with_name("rm")
                .about("Remove a git repository from the list")
                .arg(
                    Arg::with_name("repo_name")
                        .value_name("REPOSITORY_NAME")
                        .help("The repository name")
                        .takes_value(true),
                ),
        ).get_matches();

    run(cmd)
}

fn run(cmd: ArgMatches) {
    grm::update_db();

    match cmd.subcommand() {
        ("add", Some(add_matches)) => {
            let git_pattern = String::from("/**/*.git");
            let add_path = add_matches.value_of("path").unwrap_or(".");
            grm::add(String::from(add_path), git_pattern);
        }
        ("goto", Some(goto_matches)) => {
            if let Some(repo_name) = goto_matches.value_of("repo_name") {
                grm::goto(String::from(repo_name));
            }
        }
        ("list", Some(_)) => grm::list(),
        ("rm", Some(rm_matches)) => {
            if let Some(repo_name) = rm_matches.value_of("repo_name") {
                grm::rm(String::from(repo_name));
            }
        }
        ("", None) => {
            eprintln!("error : not enough argument. ");
            println!("{}", cmd.usage());
            println!("For more information try --help");
        }
        _ => unreachable!(),
    }
}
