use glob::glob;
use std::result::Result;

// TODO : Check for a database integration first! (SQLITE)

#[derive(Debug)]
pub enum GitRepo {
    Add(String),
    /*List,
    Rm (String),*/
}

impl GitRepo {
    pub fn call(&self) {
        match *self {
            GitRepo::Add(ref path) => {
                let git_pattern = String::from("/**/*.git");
                let git_repo_list = search(&path_with_pattern(path, &git_pattern));
                add(git_repo_list);
            }
        }
    }
}

#[derive(Debug)]
struct Repository {
    name: String,
    path: String,
}

impl Repository {
    fn new(name: String, path: String) -> Repository {
        Repository {
            name: name,
            path: path,
        }
    }
}

// TODO add repos to SQLite DB and make a report for added / skipped (with reason) repos
fn add(repositories: Vec<Repository>) {
    for i in 0..repositories.len() {
        println!("{:?}", repositories[i]);
    }
}

// TODO see if it's possible to exclude folders (node_modules, ...)
fn search(pattern: &String) -> Vec<Repository> {
    let mut repositories: Vec<Repository> = Vec::new();

    // TODO handle error cases for paths (parent is root path, bad path, ...)
    for path in glob(pattern).unwrap().filter_map(Result::ok) {
        let str_path = path.as_path().to_str().unwrap();
        let parent = path.parent().unwrap();
        let name = parent.iter().last().unwrap().to_str().unwrap();

        let repo = Repository::new(String::from(name), String::from(str_path));
        repositories.push(repo);
    }

    repositories
}

fn path_with_pattern(path: &String, pattern: &String) -> String {
    path.to_string() + pattern
}