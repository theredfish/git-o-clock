#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use clap::ArgMatches;
use std::process;

mod cli;
pub mod db;
mod grm;

fn main() {
    if let Err(e) = db::run_pending_migrations() {
        eprintln!("GRM update failed : {}", e);
        eprintln!("Please open an issue with your output : https://github.com/theredfish/git-repo-manager/issues/new");
        process::exit(1);
    }

    run(cli::init());
}

fn run(matches: ArgMatches) {
    match matches.subcommand() {
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
            println!("{}", matches.usage());
            println!("For more information try --help");
        }
        _ => unreachable!(),
    }
}
