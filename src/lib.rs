extern crate glob;

use glob::glob;

// TODO separate code in files
// TODO add Repository struct and impl

pub struct Config {
    pub query: String,
    pub path: String,
}

// TODO remove debug
#[derive(Debug)]
pub enum RepoManager {
    Add(String),
    /*List,
    Rm (String),*/
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let subject = args[2].clone();

        Ok(Config {
            query: query,
            path: subject,
        })
    }
}

impl RepoManager {
    pub fn call(&self) {
        match *self {
            RepoManager::Add(ref path) => {
                let git_pattern = String::from("/**/*.git");
                let git_repo_list = search(&path_with_pattern(path, &git_pattern));
                add(git_repo_list);
            }

        }
    }
}

// TODO update repositories by a Vec<Repository>
fn add(repositories: Vec<String>) {
    println!("You're in the \"add\" fn");
}

// TODO return Vec<Repository>
fn search(pattern_path: &String) -> Vec<String> {
    let repositories: Vec<String> = Vec::new();

    for entry in glob(pattern_path).unwrap() {
        if let Ok(pattern_path) = entry {
            println!("{:?}", pattern_path.display())
        }
    }

    repositories
}

fn path_with_pattern(path: &String, pattern: &String) -> String {
    path.to_string() + pattern
}
