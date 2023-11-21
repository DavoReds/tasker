use crate::{
    cli::{Cli, Command},
    config::{Configuration, Language},
};
use anyhow::anyhow;
use owo_colors::OwoColorize;
use tasker_lib::todos::{State, Task, ToDo};

fn get_last_index(to_do: &ToDo) -> usize {
    match to_do.tasks.last() {
        Some(last) => last.id + 1,
        None => 0,
    }
}

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
        Some(Command::Add(task)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;
            let index = get_last_index(&to_do);

            match task.project {
                Some(project) => to_do.add_task(
                    Task::create(task.description)
                        .id(index)
                        .project(project)
                        .tags(task.tags.unwrap_or_default())
                        .build(),
                ),
                None => to_do.add_task(
                    Task::create(task.description)
                        .id(index)
                        .tags(task.tags.unwrap_or_default())
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
                        return Err(anyhow!("Failed to save To-Do file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar el archivo de tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::AddMultiple(tasks)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;
            let mut index = get_last_index(&to_do);

            match tasks.project {
                Some(pro) => {
                    to_do
                        .tasks
                        .extend(tasks.descriptions.into_iter().map(|desc| {
                            let last_index = index;
                            index += 1;
                            Task::create(desc)
                                .id(last_index)
                                .project(pro.clone())
                                .build()
                        }));
                }
                None => {
                    to_do
                        .tasks
                        .extend(tasks.descriptions.into_iter().map(|desc| {
                            let last_index = index;
                            index += 1;
                            Task::create(desc).id(last_index).build()
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
                        return Err(anyhow!("Failed to save To-Do file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
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
                        return Err(anyhow!("Failed to save To-Do file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::Edit(edit_task)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            match to_do
                .tasks
                .iter_mut()
                .find(|task| task.id == edit_task.task)
            {
                Some(task) => {
                    if edit_task.description.is_some() {
                        task.description = edit_task.description.unwrap();
                    }

                    if edit_task.project.is_some() {
                        task.project = edit_task.project.unwrap();
                    }

                    if edit_task.state.is_some() {
                        task.state = edit_task.state.unwrap().into();
                    }

                    if edit_task.tags.is_some() {
                        task.replace_tags(edit_task.tags.unwrap());
                    }
                }
                None => match configuration.language {
                    Language::English => return Err(anyhow!("Task doesn't exist".red())),
                    Language::Spanish => return Err(anyhow!("Tarea no existe".red())),
                },
            }

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "To-Do edited".blue()),
                    Language::Spanish => println!("{}", "To-Do editado".blue()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save To-Do file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        Some(Command::Delete(tasks)) => {
            let mut to_do = ToDo::get_to_do(&configuration.to_do_path)?;

            to_do.tasks.retain(|task| !tasks.tasks.contains(&task.id));

            match to_do.save(&configuration.to_do_path) {
                Ok(_) => match configuration.language {
                    Language::English => println!("{}", "Tasks deleted".red()),
                    Language::Spanish => println!("{}", "Tareas eliminadas".red()),
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save To-Do file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de tareas: {}",
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
                        println!("{}", "Se limpiaron las tareas completadas".purple())
                    }
                },
                Err(err) => match configuration.language {
                    Language::English => {
                        return Err(anyhow!("Failed to save To-Do file: {}", err.red()))
                    }
                    Language::Spanish => {
                        return Err(anyhow!(
                            "No se pudo guardar archivo de tareas: {}",
                            err.red()
                        ))
                    }
                },
            }
        }
        None => println!("So you want to do nothing at all, huh?"),
    }

    Ok(())
}

// TODO: Create List tasks function

// fn list_tasks(to_do: ToDo, config: &Configuration) {
//     todo!()
// }
