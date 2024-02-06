use crate::{error::TaskerFailure, todos::ToDo};
use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use std::io::Write;

/// Returns an object containing the project's respective directories to store
/// data.
///
/// # Errors
///
/// Returns an error if the program can't determine the appropiate directories
/// to store its data in the user's operating system.
pub fn get_project_directories() -> Result<ProjectDirs, TaskerFailure> {
    let project_directories = ProjectDirs::from("dev", "DaliaReds", "tasker")
        .ok_or(TaskerFailure::ProjectDirectoryError(
        std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "System not supported",
        ),
    ))?;

    if !project_directories.config_dir().exists() {
        std::fs::create_dir_all(project_directories.config_dir())?;
    }

    if !project_directories.data_dir().exists() {
        std::fs::create_dir_all(project_directories.data_dir())?;
    }

    Ok(project_directories)
}

impl ToDo {
    /// Parses and returns a deserialized `ToDo` struct from the given file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the program failed to read the given file.
    pub fn get_to_do(file_path: &Utf8Path) -> Result<Self, TaskerFailure> {
        match file_path.try_exists() {
            Ok(true) => {
                Ok(ron::from_str(std::fs::read_to_string(file_path)?.as_str())?)
            }
            Ok(false) => Ok(Self::default()),
            Err(err) => Err(TaskerFailure::ProjectDirectoryError(err)),
        }
    }

    /// Returns the default path to store Tasks in.
    ///
    /// # Errors
    ///
    /// Returns an error if it failed to determine the default file paths for
    /// the program or if this path is not valid UTF-8.
    pub fn get_default_to_do_path() -> Result<Utf8PathBuf, TaskerFailure> {
        let dirs = get_project_directories()?;

        let mut config_dir =
            Utf8PathBuf::try_from(dirs.data_dir().to_path_buf())?;
        config_dir.push("todo.ron");

        Ok(config_dir)
    }

    /// Writes the Tassk into the filesystem.
    ///
    /// # Errors
    ///
    /// Returns an error if it failed to create the file at the given location,
    /// if it failed to serialize the `ToDo` struct into the RON file format or
    /// if it failed to write data into the given file path.
    pub fn save(&self, path: &Utf8Path) -> Result<(), TaskerFailure> {
        let mut to_do_file = std::fs::File::create(path)?;

        to_do_file.write_all(ron::to_string(self)?.as_bytes())?;

        Ok(())
    }
}
