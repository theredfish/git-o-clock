use clap::{App, Arg, ArgMatches, SubCommand};

pub fn init<'a>() -> ArgMatches<'a> {
    App::new("Git Repository Manager")
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
        ).get_matches()
}
