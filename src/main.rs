#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use clap::Shell;
use std::io;

mod cli;
mod config;
mod db;
mod errors;
mod grm;

use config::Config;
use db::models::Repository;
use errors::GrmError;
use grm::Grm;

fn main() {
    let config = Config::new();
    run(Grm::new(config));
}

fn run(grm: Grm) {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let path = add_matches.value_of("path").unwrap_or(".");
            print_add_result(grm.add(path.to_string()));
        }
        ("goto", Some(goto_matches)) => {
            if let Some(repo_name) = goto_matches.value_of("repo_name") {
                print_location_result(grm.location(repo_name.to_string()));
            }
        }
        ("list", Some(_)) => print_list_result(grm.list()),
        ("rm", Some(rm_matches)) => {
            if let Some(repo_name) = rm_matches.value_of("repo_name") {
                print_rm_result(grm.rm(repo_name.to_string()), repo_name.to_string());
            }
        }
        ("completions", Some(_)) => {
            cli::build_cli().gen_completions_to("grm", Shell::Bash, &mut io::stdout());
        }
        ("", None) => {
            eprintln!("error : not enough argument. ");
            println!("{}", matches.usage());
            println!("For more information try --help");
        }
        _ => unreachable!(),
    }
}

fn print_add_result(add_result: Vec<Result<Repository, GrmError>>) {
    let (repositories, errors): (Vec<_>, Vec<_>) =
        add_result.iter().partition(|result| result.is_ok());

    let repositories: Vec<&Repository> = repositories
        .iter()
        .map(|res| res.as_ref().unwrap())
        .collect();
    let errors: Vec<&GrmError> = errors.iter().map(|res| res.as_ref().unwrap_err()).collect();

    if !repositories.is_empty() {
        let repositories_count = repositories.len();
        let repositories_wording = if repositories_count > 1 {
            "repositories have been saved"
        } else {
            "repository has been saved"
        };

        println!("{} {}", repositories_count, repositories_wording);
        for repo in repositories {
            println!("{}", repo.name);
        }
    }

    if !errors.is_empty() {
        let errors_count = errors.len();
        let errors_wording = if errors_count > 1 {
            "repositories have not been saved"
        } else {
            "repository has not been saved"
        };

        eprintln!("\n{} {}", errors_count, errors_wording);
        for error in errors {
            eprintln!("{}", error);
        }
    }
}

fn print_list_result(list_result: Result<Vec<Repository>, GrmError>) {
    if let Ok(repos) = list_result {
        for repo in repos {
            println!("{}", repo.name)
        }
    } else {
        eprintln!("{}", list_result.unwrap_err());
    }
}

fn print_location_result(location_result: Result<std::path::PathBuf, GrmError>) {
    match location_result {
        Ok(location) => println!("{}", location.display()),
        Err(err) => eprintln!("{}", err),
    }
}

fn print_rm_result(rm_result: Result<usize, GrmError>, repo_name: String) {
    match rm_result {
        Ok(_) => println!("{} removed", repo_name),
        Err(err) => eprintln!("{}", err),
    }
}
