use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use lib_tasker::{error::TaskerError, io::get_project_directories};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub language: Language,
    pub to_do_path: Utf8PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Spanish,
}

impl Configuration {
    pub fn new(to_do_path: &Utf8Path) -> Result<Self, TaskerError> {
        let config_path = Self::get_default_path()?;

        match config_path.try_exists() {
            Ok(exists) => {
                if exists {
                    Ok(toml::from_str(&std::fs::read_to_string(config_path)?)?)
                } else {
                    let config = Self {
                        name: "John Doe".to_string(),
                        language: Language::default(),
                        to_do_path: to_do_path.to_owned(),
                    };

                    config.save_config()?;

                    Ok(config)
                }
            }
            Err(err) => Err(TaskerError::ProjectDirectoryError(err)),
        }
    }

    pub fn from_given_file(file_path: &Utf8Path) -> Result<Self, TaskerError> {
        match file_path.try_exists() {
            Ok(exists) => {
                if exists {
                    Ok(toml::from_str(&std::fs::read_to_string(file_path)?)?)
                } else {
                    Err(TaskerError::ProjectDirectoryError(std::io::Error::from(
                        std::io::ErrorKind::NotFound,
                    )))
                }
            }
            Err(err) => Err(TaskerError::ProjectDirectoryError(err)),
        }
    }

    pub fn get_default_path() -> Result<Utf8PathBuf, TaskerError> {
        let dirs = get_project_directories()?;

        let mut config_dir = Utf8PathBuf::try_from(dirs.config_dir().to_path_buf())?;
        config_dir.push("tasker-cli.toml");

        Ok(config_dir)
    }

    fn save_config(&self) -> Result<(), TaskerError> {
        let mut config_file = std::fs::File::create(Configuration::get_default_path()?)?;

        config_file.write_all(toml::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }
}
