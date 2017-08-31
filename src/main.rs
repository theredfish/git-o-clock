extern crate grm;

use std::env;
use std::process;
use std::path::Path;

use grm::cmd_parser::Config;
use grm::repo_manager::GitRepo;
use grm::db::*;

fn main() {
    let connection = establish_connection();
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // TODO check if the path exists before to continue

    /*let path_exists = Path::new(config.path).exists();
    match false {

    }*/

    run(config);
}

fn run(config: Config) {
    let pattern = String::from("/**/*.git");
    let git_repo = GitRepo::new(pattern);

    match config.query.as_ref() {
        "add" => git_repo.add(String::from(config.path)),
        _ => {
            println!("{} : command not found", config.query);
            process::exit(1);
        },
    }
}