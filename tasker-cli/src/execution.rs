use anyhow::anyhow;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use tasker_lib::{
    io::{get_to_do, get_to_do_path, save_to_do},
    todos::Task,
};

use crate::{
    cli::{Cli, Command},
    config::{get_config, get_config_path},
};

pub fn execute_application(cli: Cli) -> anyhow::Result<()> {
    let config_path = match cli.config_file {
        Some(path) => path,
        None => get_config_path()?,
    };

    let to_do_path = match cli.to_do_file {
        Some(path) => path,
        None => get_to_do_path()?,
    };

    let configuration = get_config(config_path, to_do_path)?;

    match cli.command {
        Some(Command::Add(task)) => {
            let mut to_do = get_to_do(&configuration.to_do_path)?;

            match task.project {
                Some(project) => to_do.add_task(
                    Task::create(task.description)
                        .project(project)
                        .tags(task.tags.unwrap_or_default())
                        .build(),
                ),
                None => to_do.add_task(
                    Task::create(task.description)
                        .tags(task.tags.unwrap_or_default())
                        .build(),
                ),
            }

            match save_to_do(&configuration.to_do_path, &to_do) {
                Ok(_) => println!("{}", "Task saved".green()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        Some(Command::AddMultiple(tasks)) => {
            let mut to_do = get_to_do(&configuration.to_do_path)?;

            to_do
                .tasks
                .par_extend(
                    tasks
                        .descriptions
                        .into_par_iter()
                        .map(|t| match &tasks.project {
                            Some(pro) => Task::create(t).project(pro).build(),
                            None => Task::create(t).build(),
                        }),
                );

            match save_to_do(&configuration.to_do_path, &to_do) {
                Ok(_) => println!("{}", "Tasks saved".green()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        Some(Command::Toggle(toggle)) => {
            let mut to_do = get_to_do(&configuration.to_do_path)?;

            to_do
                .tasks
                .par_iter_mut()
                .enumerate()
                .filter(|(idx, _)| toggle.tasks.contains(idx))
                .for_each(|(_, task)| task.change_state(toggle.state.into()));

            match save_to_do(&configuration.to_do_path, &to_do) {
                Ok(_) => println!("{}", "State changed".yellow()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        None => println!("So you want to do nothing at all, huh?"),
    }

    Ok(())
}
