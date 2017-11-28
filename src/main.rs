extern crate grm;
extern crate clap;

use clap::{Arg, App, ArgMatches, SubCommand};
use grm::repo_manager::{add, list};

fn main() {
    let cmd = App::new("Git Repository Manager")
        .version("0.1.0")
        .author("Julian Didier (theredfish)")
        .about("Git Repository Manager (GRM) helps manage GIT repositories from your terminal.")
        .subcommand(SubCommand::with_name("add")
            .about("Search and add your git repositories for a given path; default path is the \
            current directory")
            .arg(Arg::with_name("path")
                .value_name("PATH")
                .help("A relative / absolute path or a directory hierarchy.")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("list")
            .about("List your saved repositories for the given pattern")
            .arg(Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("The pattern to apply")
                .takes_value(true)))
        .get_matches();

    run(cmd)
}

fn run(cmd : ArgMatches) {
    match cmd.subcommand() {
        ("add", Some(add_matches)) => {
            let git_pattern = String::from("/**/*.git");
            let add_path = add_matches.value_of("path").unwrap_or(".");
            add(String::from(add_path), git_pattern);
        },
        ("list", Some(list_matches)) => list(),
        ("", None)           => {
            eprintln!("error : not enough argument. ");
            println!("{}", cmd.usage());
            println!("For more information try --help");
        },
        _                   => unreachable!()
    }
}
