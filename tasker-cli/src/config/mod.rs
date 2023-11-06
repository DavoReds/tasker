use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tasker_lib::{
    error::TaskerError,
    io::{get_project_directories, get_to_do_path},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub language: Language,
    pub to_do_path: PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Spanish,
}

impl Configuration {
    pub fn new(name: String, language: Language, to_do_path: PathBuf) -> Self {
        Self {
            name,
            language,
            to_do_path,
        }
    }
}

fn get_config_path(path: Option<&Path>) -> Result<PathBuf, TaskerError> {
    match path {
        Some(path) => Ok(path.to_owned()),
        None => {
            let dirs = get_project_directories()?;
            Ok(dirs.config_dir().join("tasker-cli.toml"))
        }
    }
}

pub fn get_config(
    config_path: Option<&Path>,
    to_do_path: Option<&Path>,
) -> Result<Configuration, TaskerError> {
    let config_path = get_config_path(config_path)?;

    let config = if config_path.exists() {
        toml::from_str(std::fs::read_to_string(config_path)?.as_str())?
    } else {
        let to_do_path = get_to_do_path(to_do_path)?;
        Configuration::new("John Doe".to_string(), Language::default(), to_do_path)
    };

    Ok(config)
}
