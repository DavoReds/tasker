use color_eyre::eyre::eyre;
use directories::ProjectDirs;
use std::path::PathBuf;

use tasker_lib::todos::ToDo;

pub fn extract_to_do(path: Option<PathBuf>) -> color_eyre::Result<ToDo> {
    let to_do_path = match path {
        Some(p) => p,
        None => {
            let dirs = get_project_directories()?;
            dirs.data_dir().join("todo.ron")
        }
    };

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
