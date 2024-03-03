mod helpers;

use crate::{
    cli::{
        AddMultipleToDo, AddToDo, Cli, Command, DeleteToDo, EditToDo, ListToDo,
        ToggleToDo,
    },
    config::{Configuration, Language},
};
use color_eyre::eyre::eyre;
use helpers::{get_index, list_to_dos};
use lib_tasker::{
    io::get_project_directories,
    todos::{State, Task, ToDo},
};
use owo_colors::OwoColorize;

/// Executes the application.
///
/// # Errors
///
/// Returns an error if the execution of the application failed at any point.
pub fn execute_application(cli: Cli) -> color_eyre::Result<()> {
    let configuration = if let Some(path) = cli.config_file {
        Configuration::from_given_file(&path)?
    } else {
        let to_do_path = if let Some(path) = cli.to_do_file {
            path
        } else {
            ToDo::get_default_to_do_path()?
        };

        Configuration::new(&to_do_path)?
    };

    match cli.command {
        Some(Command::Add(add)) => add_task(add, &configuration)?,
        Some(Command::AddMultiple(add)) => {
            add_multiple_tasks(add, &configuration)?;
        }
        Some(Command::Clean) => clean_completed_tasks(&configuration)?,
        Some(Command::Delete(delete)) => delete_tasks(&delete, &configuration)?,
        Some(Command::Edit(edit)) => edit_task(edit, &configuration)?,
        Some(Command::List(list)) => list_tasks(list, &configuration)?,
        Some(Command::Paths) => get_paths()?,
        Some(Command::Toggle(toggle)) => toggle_tasks(&toggle, &configuration)?,
        None => {
            let to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            list_to_dos(to_do, &configuration, None);
        }
    }

    Ok(())
}

fn add_task(to_add: AddToDo, config: &Configuration) -> color_eyre::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;
    let index = get_index(&to_do);

    match to_add.project {
        Some(project) => to_do.add_task(
            Task::create(to_add.description)
                .id(index)
                .project(project)
                .tags(to_add.tag.unwrap_or_default())
                .build(),
        ),
        None => to_do.add_task(
            Task::create(to_add.description)
                .id(index)
                .tags(to_add.tag.unwrap_or_default())
                .build(),
        ),
    }

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => println!("{}", "Added Task".green()),
            Language::Spanish => {
                println!("{}", "Tarea añadida".green());
            }
        },
        Err(err) => match config.language {
            Language::English => {
                return Err(eyre!("Failed to save Task file: {}", err.red()))
            }
            Language::Spanish => {
                return Err(eyre!(
                    "No se pudo guardar el archivo de Tareas: {}",
                    err.red()
                ))
            }
        },
    }

    Ok(())
}

fn add_multiple_tasks(
    to_add: AddMultipleToDo,
    config: &Configuration,
) -> color_eyre::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;
    let mut index = get_index(&to_do);

    match to_add.project {
        Some(project) => {
            to_do
                .tasks
                .extend(to_add.descriptions.into_iter().map(|desc| {
                    let last_index = index;
                    index += 1;
                    Task::create(desc)
                        .id(last_index)
                        .project(project.clone())
                        .tags(to_add.tag.clone().unwrap_or_default())
                        .build()
                }));
        }
        None => {
            to_do
                .tasks
                .extend(to_add.descriptions.into_iter().map(|desc| {
                    let last_index = index;
                    index += 1;
                    Task::create(desc)
                        .id(last_index)
                        .tags(to_add.tag.clone().unwrap_or_default())
                        .build()
                }));
        }
    }

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => println!("{}", "Added Tasks".green()),
            Language::Spanish => {
                println!("{}", "Tareas añadidas".green());
            }
        },
        Err(err) => match config.language {
            Language::English => {
                return Err(eyre!("Failed to save Task file: {}", err.red()))
            }
            Language::Spanish => {
                return Err(eyre!(
                    "No se pudo guardar archivo de Tareas: {}",
                    err.red()
                ))
            }
        },
    }

    Ok(())
}

fn clean_completed_tasks(config: &Configuration) -> color_eyre::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;

    to_do.tasks.retain(|task| task.state != State::Done);

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => {
                println!("{}", "Cleaned completed tasks".purple());
            }
            Language::Spanish => {
                println!("{}", "Se limpiaron las Tareas completadas".purple());
            }
        },
        Err(err) => match config.language {
            Language::English => {
                return Err(eyre!("Failed to save Task file: {}", err.red()))
            }
            Language::Spanish => {
                return Err(eyre!(
                    "No se pudo guardar archivo de Tareas: {}",
                    err.red()
                ))
            }
        },
    }

    Ok(())
}

fn delete_tasks(
    to_delete: &DeleteToDo,
    config: &Configuration,
) -> color_eyre::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;

    to_do
        .tasks
        .retain(|task| !to_delete.tasks.contains(&task.id));

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => println!("{}", "Deleted Tasks".red()),
            Language::Spanish => {
                println!("{}", "Tareas eliminadas".red());
            }
        },
        Err(err) => match config.language {
            Language::English => {
                return Err(eyre!("Failed to save Task file: {}", err.red()))
            }
            Language::Spanish => {
                return Err(eyre!(
                    "No se pudo guardar archivo de Tareas: {}",
                    err.red()
                ))
            }
        },
    }

    Ok(())
}

fn edit_task(
    to_edit: EditToDo,
    config: &Configuration,
) -> color_eyre::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;

    match to_do.tasks.iter_mut().find(|task| task.id == to_edit.task) {
        Some(task) => {
            if let Some(description) = to_edit.description {
                task.description = description;
            }

            if let Some(project) = to_edit.project {
                task.project = project;
            }

            if let Some(state) = to_edit.state {
                task.state = state.into();
            }

            if let Some(tags) = to_edit.tags {
                task.replace_tags(tags);
            }
        }
        None => match config.language {
            Language::English => return Err(eyre!("Task doesn't exist".red())),
            Language::Spanish => return Err(eyre!("Tarea no existe".red())),
        },
    }

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => println!("{}", "to_edited Task".blue()),
            Language::Spanish => println!("{}", "Tarea to_editada".blue()),
        },
        Err(err) => match config.language {
            Language::English => {
                return Err(eyre!("Failed to save Task file: {}", err.red()))
            }
            Language::Spanish => {
                return Err(eyre!(
                    "No se pudo guardar archivo de Tareas: {}",
                    err.red()
                ))
            }
        },
    }

    Ok(())
}

fn list_tasks(
    to_list: ListToDo,
    config: &Configuration,
) -> color_eyre::Result<()> {
    let to_do = ToDo::get_to_do(&config.to_do_path)?;
    list_to_dos(to_do, config, Some(to_list));

    Ok(())
}

fn get_paths() -> color_eyre::Result<()> {
    let paths = get_project_directories()?;

    println!("Config path: {}", paths.config_dir().display());
    println!("Data path: {}", paths.data_dir().display());

    Ok(())
}

fn toggle_tasks(
    to_toggle: &ToggleToDo,
    config: &Configuration,
) -> color_eyre::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;

    to_do
        .tasks
        .iter_mut()
        .filter(|task| to_toggle.tasks.contains(&task.id))
        .for_each(|task| task.change_state(to_toggle.state.into()));

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => {
                println!("{}", "State changed".yellow());
            }
            Language::Spanish => {
                println!("{}", "Estado cambiado".yellow());
            }
        },
        Err(err) => match config.language {
            Language::English => {
                return Err(eyre!("Failed to save Task file: {}", err.red()))
            }
            Language::Spanish => {
                return Err(eyre!(
                    "No se pudo guardar archivo de Tareas: {}",
                    err.red()
                ))
            }
        },
    }

    Ok(())
}
