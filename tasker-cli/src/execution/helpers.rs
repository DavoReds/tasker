use std::collections::HashSet;

use crate::{
    cli::{ListToDo, SortToDo},
    config::{Configuration, Language},
};
use itertools::Itertools;
use owo_colors::OwoColorize;
use tasker_lib::todos::{State, Task, ToDo};

pub fn get_index(to_do: &ToDo) -> usize {
    match to_do.tasks.last() {
        Some(last) => last.id + 1,
        None => 0,
    }
}

fn push_task(task: &Task, string: &mut String, config: &Configuration) {
    string.push_str(&format!("{}. {}\n", task.id.purple(), task.description));

    match config.language {
        Language::English => match task.state {
            State::ToDo => string.push_str(&format!("[{}] ", "To-Do".blue())),
            State::Doing => string.push_str(&format!("[{}] ", "Doing".yellow())),
            State::Done => string.push_str(&format!("[{}] ", "Done".green())),
            State::Waiting => string.push_str(&format!("[{}] ", "Waiting".red())),
        },
        Language::Spanish => match task.state {
            State::ToDo => string.push_str(&format!("[{}] ", "Por Hacer".blue())),
            State::Doing => string.push_str(&format!("[{}] ", "Haciendo".yellow())),
            State::Done => string.push_str(&format!("[{}] ", "Hecho".green())),
            State::Waiting => string.push_str(&format!("[{}] ", "Esperando".red())),
        },
    }

    string.push_str("{ ");
    let tags = task.tags.iter().join(", ");
    string.push_str(&tags);
    string.push_str(" }\n\n");
}

pub fn list_tasks(to_do: ToDo, config: &Configuration, args: Option<ListToDo>) {
    let mut output = String::new();

    match args {
        Some(options) => {
            let mut tasks = to_do.tasks;

            if let Some(description) = options.description {
                tasks.retain(|task| task.description.contains(&description));
            }

            if let Some(state) = options.state {
                tasks.retain(|task| task.state == state.into());
            }

            if let Some(tags) = options.tag {
                let tags: HashSet<String> = tags.into_iter().collect();
                tasks.retain(|task| task.tags.intersection(&tags).count() > 0);
            }

            if let Some(project) = options.project {
                tasks.retain(|task| task.project.contains(&project));
            }

            if let Some(sort_options) = options.sort_by {
                match sort_options {
                    SortToDo::Description => tasks.sort_unstable_by(|a, b| {
                        a.description
                            .to_lowercase()
                            .cmp(&b.description.to_lowercase())
                    }),
                    // FIXME: I could print the projects here, to the sorting
                    // makes sense visually
                    SortToDo::Project => tasks.sort_unstable_by(|a, b| {
                        a.project.to_lowercase().cmp(&b.project.to_lowercase())
                    }),
                    SortToDo::ID => tasks.sort_unstable_by(|a, b| a.id.cmp(&b.id)),
                    SortToDo::State => tasks.sort_unstable_by(|a, b| a.state.cmp(&b.state)),
                }
            }

            tasks
                .iter()
                .for_each(|task| push_task(task, &mut output, config));
        }
        None => {
            match config.language {
                Language::English => output.push_str(&format!(
                    "Hello, {}!\nHere's what you got for today:\n",
                    config.name
                )),
                Language::Spanish => output.push_str(&format!(
                    "¡Hola, {}!\nEsto es lo que tienes para hoy:\n",
                    config.name
                )),
            }

            output.push('\n');

            let projects = to_do
                .tasks
                .iter()
                .unique_by(|task| &task.project)
                .map(|task| task.project.clone())
                .sorted();

            for project in projects {
                output.push_str(&format!("{}\n\n", project.purple().underline()));

                to_do
                    .tasks
                    .iter()
                    .filter(|task| task.project == project)
                    .for_each(|task| push_task(task, &mut output, config));
            }
        }
    }

    print!("{output}");
}
