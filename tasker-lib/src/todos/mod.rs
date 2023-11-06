pub mod tasks;
pub use tasks::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ToDo {
    pub tasks: Vec<Task>,
}

impl From<Vec<Task>> for ToDo {
    fn from(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }
}
