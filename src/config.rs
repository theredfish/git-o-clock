use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub max_depth_search: usize,
    pub ignored_folders: Option<Vec<String>>,
}

impl Config {
    pub fn new() -> Self {
        let config_path = match env::var_os("GRM_CONFIG_PATH") {
            None => return Config::default(),
            Some(config_path) => {
                if config_path.is_empty() {
                    return Config::default();
                }

                PathBuf::from(config_path)
            }
        };

        match parse(config_path) {
            Ok(config) => config,
            Err(err) => {
                eprintln!(
                    "Cannot read or load the configuration file. The default configuration will be used. {}",
                    err
                );

                Config::default()
            }
        }
    }
}

fn parse<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);
    // Read the JSON contents of the file as an instance of `Config`
    let config: Config = serde_json::from_reader(reader)?;

    // Return the `Config`
    Ok(config)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_depth_search: 2,
            ignored_folders: None,
        }
    }
}
