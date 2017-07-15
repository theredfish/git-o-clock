extern crate grm;

use std::env;
use std::process;
use std::path::Path;

use grm::Config;
use grm::RepoManager;

fn main() {
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
        "add" => RepoManager::Add(String::from(config.path)).call(),
        _ => {
            println!("{} : command not found", config.query);
            process::exit(1);
        },
    }
}