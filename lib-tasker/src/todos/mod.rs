pub mod tasks;
pub use tasks::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct ToDo {
    pub tasks: Vec<Task>,
}

impl From<Vec<Task>> for ToDo {
    fn from(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }
}

impl ToDo {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_task_works() {
        let mut todo = ToDo::from(vec![Task::create("This is a test")
            .project("Testing")
            .build()]);

        todo.add_task(Task::create("This is from a method").build());

        assert_eq!(
            todo,
            ToDo {
                tasks: vec![
                    Task::create("This is a test").project("Testing").build(),
                    Task::create("This is from a method").build()
                ]
            }
        );
    }
}

