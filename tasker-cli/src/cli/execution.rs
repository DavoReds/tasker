use color_eyre::eyre::eyre;
use directories::ProjectDirs;
use std::{io::Write, path::PathBuf};

use tasker_lib::todos::ToDo;

fn get_to_do_path(path: Option<PathBuf>) -> color_eyre::Result<PathBuf> {
    match path {
        Some(p) => Ok(p),
        None => {
            let dirs = get_project_directories()?;
            Ok(dirs.data_dir().join("todo.ron"))
        }
    }
}

pub fn extract_to_do(path: Option<PathBuf>) -> color_eyre::Result<ToDo> {
    let to_do_path = get_to_do_path(path)?;

    let to_do = if to_do_path.exists() {
        ron::from_str(std::fs::read_to_string(to_do_path)?.as_str())?
    } else {
        ToDo::default()
    };

    Ok(to_do)
}

pub fn get_project_directories() -> color_eyre::Result<ProjectDirs> {
    let project_directories = ProjectDirs::from("dev", "DaliaReds", "tasker-cli")
        .ok_or(eyre!("Project directories not supported on this system"))?;

    if !project_directories.config_dir().exists() {
        std::fs::create_dir_all(project_directories.config_dir())?;
    }

    if !project_directories.data_dir().exists() {
        std::fs::create_dir_all(project_directories.data_dir())?;
    }

    Ok(project_directories)
}

pub fn save_to_do(path: Option<PathBuf>, todo: &ToDo) -> color_eyre::Result<()> {
    let to_do_path = get_to_do_path(path)?;

    let mut to_do_file = std::fs::File::create(to_do_path)?;

    to_do_file.write_all(ron::to_string(todo)?.as_bytes())?;

    Ok(())
}
