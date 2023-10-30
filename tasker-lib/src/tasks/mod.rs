use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Task {
    pub description: String,
    pub state: State,
    pub tags: Vec<String>,
    pub project: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq, Clone)]
pub enum State {
    #[default]
    ToDo,
    Doing,
    Done,
    Waiting,
}

impl Task {
    pub fn create(description: &str) -> TaskBuilder {
        TaskBuilder {
            description: description.to_string(),
            state: State::default(),
            tags: None,
            project: None,
        }
    }
}

#[derive(Debug)]
pub struct TaskBuilder {
    description: String,
    state: State,
    tags: Option<Vec<String>>,
    project: Option<String>,
}

impl TaskBuilder {
    pub fn state(&mut self, state: State) -> &mut Self {
        self.state = state;
        self
    }

    pub fn project(&mut self, project: impl Into<String>) -> &mut Self {
        self.project = Some(project.into());
        self
    }

    pub fn tag(&mut self, tag: impl Into<String>) -> &mut Self {
        if self.tags.is_none() {
            self.tags = Some(vec![tag.into()]);
        } else {
            self.tags.as_mut().unwrap().push(tag.into());
        }

        self
    }

    pub fn tags(&mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.tags = Some(tags.into_iter().map(Into::into).collect());
        self
    }

    pub fn build(&self) -> Task {
        Task {
            description: self.description.clone(),
            state: self.state.clone(),
            tags: self.tags.clone().unwrap_or_default(),
            project: self.project.clone().unwrap_or("Inbox".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_builder_works() {
        let task = Task::create("This is a test").build();

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: vec![],
                project: "Inbox".to_string()
            }
        );
    }

    #[test]
    fn task_builder_change_state_works() {
        let task = Task::create("This is a test").state(State::Waiting).build();

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::Waiting,
                tags: vec![],
                project: "Inbox".to_string()
            }
        );
    }

    #[test]
    fn task_builder_change_tag_works() {
        let task = Task::create("This is a test").tag("Test").build();

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: vec!["Test".into()],
                project: "Inbox".to_string()
            }
        );
    }

    #[test]
    fn task_builder_change_tags_works() {
        let task = Task::create("This is a test")
            .tags(vec!["Test 1", "Test 2"])
            .build();

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: vec!["Test 1".into(), "Test 2".into()],
                project: "Inbox".to_string()
            }
        );
    }

    #[test]
    fn task_builder_change_project_works() {
        let task = Task::create("This is a test").project("Testing").build();

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: vec![],
                project: "Testing".to_string()
            }
        );
    }
}
