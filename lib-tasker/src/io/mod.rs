use crate::{error::TaskerError, todos::ToDo};
use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use std::io::Write;

pub fn get_project_directories() -> Result<ProjectDirs, TaskerError> {
    let project_directories = ProjectDirs::from("dev", "DaliaReds", "tasker")
        .ok_or(TaskerError::ProjectDirectoryError(
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
    pub fn get_to_do(file_path: &Utf8Path) -> Result<Self, TaskerError> {
        match file_path.try_exists() {
            Ok(exists) => {
                if exists {
                    Ok(ron::from_str(
                        std::fs::read_to_string(file_path)?.as_str(),
                    )?)
                } else {
                    Ok(Self::default())
                }
            }
            Err(err) => Err(TaskerError::ProjectDirectoryError(err)),
        }
    }

    pub fn get_default_to_do_path() -> Result<Utf8PathBuf, TaskerError> {
        let dirs = get_project_directories()?;

        let mut config_dir =
            Utf8PathBuf::try_from(dirs.data_dir().to_path_buf())?;
        config_dir.push("todo.ron");

        Ok(config_dir)
    }

    pub fn save(&self, path: &Utf8Path) -> Result<(), TaskerError> {
        let mut to_do_file = std::fs::File::create(path)?;

        to_do_file.write_all(ron::to_string(self)?.as_bytes())?;

        Ok(())
    }
}
