use glob::glob;

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