pub mod helpers;
use self::helpers::{get_index, list_tasks};
use crate::{
    cli::{Cli, Command},
    config::{Configuration, Language},
};
use anyhow::anyhow;
use owo_colors::OwoColorize;
use tasker_lib::todos::{State, Task, ToDo};

pub fn execute_application(cli: Cli) -> anyhow::Result<()> {
    let configuration = match cli.config_file {
        Some(path) => Configuration::from_given_file(&path)?,
        None => {
            let to_do_path = match cli.to_do_file {
                Some(path) => path,
                None => ToDo::get_default_to_do_path()?,
            };

            Configuration::new(&to_do_path)?
        }
    };

    match cli.command {
        Some(Command::Add(add)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;
            let index = get_index(&to_do);

            match add.project {
                Some(project) => to_do.add_task(
                    Task::create(add.description)
                        .id(index)
                        .project(project)
                        .tags(add.tag.unwrap_or_default())
                        .build(),
                ),
                None => to_do.add_task(
                    Task::create(add.description)
                        .id(index)
                        .tags(add.tag.unwrap_or_default())
                        .build(),
                ),
            }

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "Task added".green()),
                    Language::Spanish => println!("{}", "Tarea añadida".green()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save Task file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar el archivo de Tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::AddMultiple(add)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;
            let mut index = get_index(&to_do);

            match add.project {
                Some(project) => {
                    to_do.tasks.extend(add.descriptions.into_iter().map(|desc| {
                        let last_index = index;
                        index += 1;
                        Task::create(desc)
                            .id(last_index)
                            .project(project.clone())
                            .tags(add.tag.clone().unwrap_or_default())
                            .build()
                    }));
                }
                None => {
                    to_do.tasks.extend(add.descriptions.into_iter().map(|desc| {
                        let last_index = index;
                        index += 1;
                        Task::create(desc)
                            .id(last_index)
                            .tags(add.tag.clone().unwrap_or_default())
                            .build()
                    }));
                }
            }

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "Tasks added".green()),
                    Language::Spanish => println!("{}", "Tareas añadidas".green()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save Task file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de Tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::Clean) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            to_do.tasks.retain(|task| task.state != State::Done);

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "Cleaned completed tasks".purple()),
                    Language::Spanish => {
                        println!("{}", "Se limpiaron las Tareas completadas".purple())
                    }
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save Task file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de Tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::Delete(delete)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            to_do.tasks.retain(|task| !delete.tasks.contains(&task.id));

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "Tasks deleted".red()),
                    Language::Spanish => println!("{}", "Tareas eliminadas".red()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save Task file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de Tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::Edit(edit)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            match to_do.tasks.iter_mut().find(|task| task.id == edit.task) {
                Some(task) => {
                    if edit.description.is_some() {
                        task.description = edit.description.unwrap();
                    }

                    if edit.project.is_some() {
                        task.project = edit.project.unwrap();
                    }

                    if edit.state.is_some() {
                        task.state = edit.state.unwrap().into();
                    }

                    if edit.tags.is_some() {
                        task.replace_tags(edit.tags.unwrap());
                    }
                }
                None => match configuration.language {
                    Language::English => return Err(anyhow!("Task doesn't exist".red())),
                    Language::Spanish => return Err(anyhow!("Tarea no existe".red())),
                },
            }

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "Task edited".blue()),
                    Language::Spanish => println!("{}", "Tarea editada".blue()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save Task file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de Tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::List(list)) => {
            let to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            list_tasks(to_do, &configuration, Some(list));
        }
        Some(Command::Toggle(toggle)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            to_do
                .tasks
                .iter_mut()
                .filter(|task| toggle.tasks.contains(&task.id))
                .for_each(|task| task.change_state(toggle.state.into()));

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "State changed".yellow()),
                    Language::Spanish => println!("{}", "Estado cambiado".yellow()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save Task file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de Tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        None => {
            let to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            list_tasks(to_do, &configuration, None);
        }
    }

    Ok(())
}
