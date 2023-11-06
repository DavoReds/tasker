use anyhow::anyhow;
use clap::Parser;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use tasker_cli::cli::{Cli, Command};
use tasker_lib::{
    io::{get_to_do, save_to_do},
    todos::Task,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Add(task)) => {
            let mut to_do = get_to_do(cli.todo_file.as_deref())?;

            match task.project {
                Some(pro) => to_do.add_task(
                    Task::create(task.description)
                        .project(pro)
                        .tags(task.tags.unwrap_or_default())
                        .build(),
                ),
                None => to_do.add_task(
                    Task::create(task.description)
                        .tags(task.tags.unwrap_or_default())
                        .build(),
                ),
            }

            match save_to_do(cli.todo_file.as_deref(), &to_do) {
                Ok(_) => println!("{}", "Task saved".green()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        Some(Command::AddMultiple(tasks)) => {
            let mut to_do = get_to_do(cli.todo_file.as_deref())?;

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

            match save_to_do(cli.todo_file.as_deref(), &to_do) {
                Ok(_) => println!("{}", "Tasks saved".green()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        Some(Command::Toggle(toggle)) => {
            let mut to_do = get_to_do(cli.todo_file.as_deref())?;

            to_do
                .tasks
                .par_iter_mut()
                .enumerate()
                .filter(|(idx, _)| toggle.tasks.contains(idx))
                .for_each(|(_, task)| task.change_state(toggle.state.into()));

            match save_to_do(cli.todo_file.as_deref(), &to_do) {
                Ok(_) => println!("{}", "State changed".yellow()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        None => println!("So you want to do nothing at all, huh?"),
    }

    Ok(())
}
