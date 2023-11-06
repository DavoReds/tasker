use std::{
    io::Write,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

use crate::{error::TaskerError, todos::ToDo};

fn get_project_directories() -> Result<ProjectDirs, TaskerError> {
    let project_directories = ProjectDirs::from("dev", "DaliaReds", "tasker-cli").ok_or(
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

fn get_to_do_path(path: Option<&Path>) -> Result<PathBuf, TaskerError> {
    match path {
        Some(p) => Ok(p.to_owned()),
        None => {
            let dirs = get_project_directories()?;
            Ok(dirs.data_dir().join("todo.ron"))
        }
    }
}

pub fn get_to_do(path: Option<&Path>) -> Result<ToDo, TaskerError> {
    let to_do_path = get_to_do_path(path)?;

    let to_do = if to_do_path.exists() {
        ron::from_str(std::fs::read_to_string(to_do_path)?.as_str())?
    } else {
        ToDo::default()
    };

    Ok(to_do)
}

pub fn save_to_do(path: Option<&Path>, todo: &ToDo) -> Result<(), TaskerError> {
    let to_do_path = get_to_do_path(path)?;

    let mut to_do_file = std::fs::File::create(to_do_path)?;

    to_do_file.write_all(ron::to_string(todo)?.as_bytes())?;

    Ok(())
}
