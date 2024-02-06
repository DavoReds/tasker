use camino::{Utf8Path, Utf8PathBuf};
use lib_tasker::{error::TaskerFailure, io::get_project_directories};
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    /// Returns a new configuration struct.
    ///
    /// # Errors
    ///
    /// Returns an error if it fails to determine the default paths for application
    /// data, if it fails to deserialize the configuration file or if it fails
    /// to determine the existence of the configuration path.
    pub fn new(to_do_path: &Utf8Path) -> Result<Self, TaskerFailure> {
        let config_path = Self::get_default_path()?;

        match config_path.try_exists() {
            Ok(true) => {
                Ok(toml::from_str(&std::fs::read_to_string(config_path)?)?)
            }
            Ok(false) => {
                let config = Self {
                    name: "John Doe".to_string(),
                    language: Language::default(),
                    to_do_path: to_do_path.to_owned(),
                };

                config.save_config()?;

                Ok(config)
            }
            Err(err) => Err(TaskerFailure::ProjectDirectoryError(err)),
        }
    }

    /// Deserializes a configuration struct from the given file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the given file path doesn't exist, or if it fails to
    /// determine its existence.
    pub fn from_given_file(
        file_path: &Utf8Path,
    ) -> Result<Self, TaskerFailure> {
        match file_path.try_exists() {
            Ok(true) => {
                Ok(toml::from_str(&std::fs::read_to_string(file_path)?)?)
            }
            Ok(false) => Err(TaskerFailure::ProjectDirectoryError(
                std::io::Error::from(std::io::ErrorKind::NotFound),
            )),
            Err(err) => Err(TaskerFailure::ProjectDirectoryError(err)),
        }
    }

    /// Returns the default configuration path.
    ///
    /// # Errors
    ///
    /// Returns an error if it failes to determine the default configuration path
    /// for the application or if said path is invalid UTF-8.
    pub fn get_default_path() -> Result<Utf8PathBuf, TaskerFailure> {
        let dirs = get_project_directories()?;

        let mut config_dir =
            Utf8PathBuf::try_from(dirs.config_dir().to_path_buf())?;
        config_dir.push("tasker-cli.toml");

        Ok(config_dir)
    }

    fn save_config(&self) -> Result<(), TaskerFailure> {
        let mut config_file = std::fs::File::create(Self::get_default_path()?)?;

        config_file.write_all(toml::to_string_pretty(self)?.as_bytes())?;

        Ok(())
    }
}
