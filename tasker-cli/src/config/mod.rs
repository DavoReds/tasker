use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tasker_lib::{error::TaskerError, io::get_project_directories};

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

pub fn get_config_path() -> Result<PathBuf, TaskerError> {
    let dirs = get_project_directories()?;
    Ok(dirs.config_dir().join("tasker-cli.toml"))
}

pub fn get_config(config_path: PathBuf, to_do_path: PathBuf) -> Result<Configuration, TaskerError> {
    let config = if config_path.exists() {
        toml::from_str(std::fs::read_to_string(config_path)?.as_str())?
    } else {
        Configuration::new("John Doe".to_string(), Language::default(), to_do_path)
    };

    Ok(config)
}
