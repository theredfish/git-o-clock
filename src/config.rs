use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub max_depth_search: u32,
    pub ignored_folders: Option<Vec<String>>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        // Open the file in read-only mode with buffer
        let json_file = File::open(path)?;
        let reader = BufReader::new(json_file);
        // Read the JSON contents of the file as an instance of `Config`
        let config: Config = serde_json::from_reader(reader)?;

        // Return the `Config`
        Ok(config)
    }

    pub fn depth_to_glob(self) -> String {
        let mut git_glob = "/".to_string();

        for _ in 1..self.max_depth_search {
            git_glob += "*/";
        }

        git_glob += "*.git";
        git_glob
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_depth_search: 2,
            ignored_folders: None,
        }
    }
}
