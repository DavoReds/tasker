use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub description: String,
    pub state: State,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum State {
    #[default]
    ToDo,
    Doing,
    Done,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            state: State::default(),
            tags: None,
        }
    }
}

#[derive(Debug)]
pub struct TaskBuilder {
    pub description: String,
    pub state: State,
    pub tags: Option<Vec<String>>,
}
