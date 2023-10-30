use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Builder)]
#[builder(setter(into))]
pub struct Task {
    pub description: String,

    #[builder(default)]
    pub state: State,

    #[builder(default = "Vec::new()")]
    pub tags: Vec<String>,

    #[builder(default = "self.default_project()")]
    pub project: String,
}

impl Task {
    pub fn create(description: &str) -> TaskBuilder {
        let mut task_builder = TaskBuilder::create_empty();
        task_builder.description = Some(description.to_string());

        task_builder
    }
}

impl TaskBuilder {
    fn default_project(&self) -> String {
        "Inbox".to_string()
    }
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq, Clone)]
pub enum State {
    #[default]
    ToDo,
    Doing,
    Done,
    Waiting,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_builder_works() -> Result<(), TaskBuilderError> {
        let task = Task::create("This is a test").build()?;

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: Vec::new(),
                project: "Inbox".to_string()
            }
        );

        Ok(())
    }
}
