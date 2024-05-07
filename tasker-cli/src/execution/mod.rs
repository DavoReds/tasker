mod helpers;

use crate::{
    cli::{
        AddTasks, Cli, Command, DeleteTasks, EditTask, ListTasks, ToggleTasks,
    },
    config::{Configuration, Language},
};
use anyhow::bail;
use helpers::{get_next_index, list_to_dos};
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
pub fn execute_application(cli: Cli) -> anyhow::Result<()> {
    let configuration = if let Some(path) = cli.config_file {
        Configuration::from_given_file(&path)?
    } else {
        let to_do_path = if let Some(path) = cli.todo_file {
            path
        } else {
            ToDo::get_default_to_do_path()?
        };

        Configuration::new(&to_do_path)?
    };

    match cli.command {
        Some(Command::Add(add)) => add_tasks(add, &configuration)?,
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

fn add_tasks(to_add: AddTasks, config: &Configuration) -> anyhow::Result<()> {
    let mut to_do = ToDo::get_to_do(&config.to_do_path)?;
    let mut next_index = get_next_index(&to_do);

    match to_add.project {
        Some(project) => {
            to_do
                .tasks
                .extend(to_add.descriptions.into_iter().map(|desc| {
                    let index = next_index;
                    next_index += 1;

                    Task::create(desc)
                        .id(index)
                        .project(project.clone())
                        .tags(to_add.tag.clone().unwrap_or_default())
                        .build()
                }));
        }
        None => {
            to_do
                .tasks
                .extend(to_add.descriptions.into_iter().map(|desc| {
                    let index = next_index;
                    next_index += 1;

                    Task::create(desc)
                        .id(index)
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
                bail!("Failed to save Task file: {}", err.red())
            }
            Language::Spanish => {
                bail!("No se pudo guardar archivo de Tareas: {}", err.red())
            }
        },
    }

    Ok(())
}

fn clean_completed_tasks(config: &Configuration) -> anyhow::Result<()> {
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
                bail!("Failed to save Task file: {}", err.red())
            }
            Language::Spanish => {
                bail!("No se pudo guardar archivo de Tareas: {}", err.red())
            }
        },
    }

    Ok(())
}

fn delete_tasks(
    to_delete: &DeleteTasks,
    config: &Configuration,
) -> anyhow::Result<()> {
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
                bail!("Failed to save Task file: {}", err.red())
            }
            Language::Spanish => {
                bail!("No se pudo guardar archivo de Tareas: {}", err.red())
            }
        },
    }

    Ok(())
}

fn edit_task(to_edit: EditTask, config: &Configuration) -> anyhow::Result<()> {
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
            Language::English => bail!("Task doesn't exist".red()),
            Language::Spanish => bail!("Tarea no existe".red()),
        },
    }

    match to_do.save(&config.to_do_path) {
        Ok(()) => match config.language {
            Language::English => println!("{}", "Edited Task".blue()),
            Language::Spanish => println!("{}", "Tarea editada".blue()),
        },
        Err(err) => match config.language {
            Language::English => {
                bail!("Failed to save Task file: {}", err.red())
            }
            Language::Spanish => {
                bail!("No se pudo guardar archivo de Tareas: {}", err.red())
            }
        },
    }

    Ok(())
}

fn list_tasks(
    to_list: ListTasks,
    config: &Configuration,
) -> anyhow::Result<()> {
    let to_do = ToDo::get_to_do(&config.to_do_path)?;
    list_to_dos(to_do, config, Some(to_list));

    Ok(())
}

fn get_paths() -> anyhow::Result<()> {
    let paths = get_project_directories()?;

    println!("Config path: {}", paths.config_dir().display());
    println!("Data path: {}", paths.data_dir().display());

    Ok(())
}

fn toggle_tasks(
    to_toggle: &ToggleTasks,
    config: &Configuration,
) -> anyhow::Result<()> {
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
                bail!("Failed to save Task file: {}", err.red())
            }
            Language::Spanish => {
                bail!("No se pudo guardar archivo de Tareas: {}", err.red())
            }
        },
    }

    Ok(())
}
