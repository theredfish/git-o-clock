extern crate grm;

use std::env;
use std::process;
use std::path::Path;

use grm::cmd_parser::Config;
use grm::repo_manager::GitRepo;
use grm::database::SQLite;

fn main() {
    let db = SQLite::new().unwrap_or_else(|err| {
        println!("SQLite database error : {} ", err);
        process::exit(1);
    });

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
    match config.query.as_ref() {
        "add" => GitRepo::Add(String::from(config.path)).call(),
        _ => {
            println!("{} : command not found", config.query);
            process::exit(1);
        },
    }
}