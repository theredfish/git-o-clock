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
mod grm;

use config::Config;
use grm::Grm;

fn main() {
    run(Grm::new(Config::new()));
}

fn run(grm: Grm) {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let path = add_matches.value_of("path").unwrap_or(".");
            grm.add(path.to_string());
        }
        ("goto", Some(goto_matches)) => {
            if let Some(repo_name) = goto_matches.value_of("repo_name") {
                grm.goto(String::from(repo_name));
            }
        }
        ("list", Some(_)) => grm.list(),
        ("rm", Some(rm_matches)) => {
            if let Some(repo_name) = rm_matches.value_of("repo_name") {
                grm.rm(String::from(repo_name));
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
