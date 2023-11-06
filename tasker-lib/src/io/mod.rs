use std::{
    io::Write,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use crate::{error::TaskerError, todos::ToDo};

pub fn get_project_directories() -> Result<ProjectDirs, TaskerError> {
    let project_directories = ProjectDirs::from("dev", "DaliaReds", "tasker").ok_or(
        TaskerError::ProjectDirectoryError(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "System not supported",
        )),
    )?;

    if !project_directories.config_dir().exists() {
        std::fs::create_dir_all(project_directories.config_dir())?;
    }

    if !project_directories.data_dir().exists() {
        std::fs::create_dir_all(project_directories.data_dir())?;
    }

    Ok(project_directories)
}

pub fn get_to_do_path() -> Result<PathBuf, TaskerError> {
    let dirs = get_project_directories()?;
    Ok(dirs.data_dir().join("todo.ron"))
}

pub fn get_to_do(path: &Path) -> Result<ToDo, TaskerError> {
    let to_do = if path.exists() {
        ron::from_str(std::fs::read_to_string(path)?.as_str())?
    } else {
        ToDo::default()
    };

    Ok(to_do)
}

pub fn save_to_do(path: &Path, todo: &ToDo) -> Result<(), TaskerError> {
    let mut to_do_file = std::fs::File::create(path)?;

    to_do_file.write_all(ron::to_string(todo)?.as_bytes())?;

    Ok(())
}
