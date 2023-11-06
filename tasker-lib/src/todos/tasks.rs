use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use str_slug::slug;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Task {
    pub description: String,
    pub state: State,
    pub tags: HashSet<String>,
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
    pub fn create(description: impl Into<String>) -> TaskBuilder {
        TaskBuilder {
            description: description.into(),
            state: State::default(),
            tags: None,
            project: None,
        }
    }

    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.insert(slug(tag.into()));
    }

    pub fn add_tags(&mut self, tags: impl IntoIterator<Item = impl Into<String>>) {
        self.tags
            .extend(tags.into_iter().map(|tag| slug(tag.into())));
    }

    pub fn change_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn change_state(&mut self, state: State) {
        self.state = state;
    }
}

#[derive(Debug)]
pub struct TaskBuilder {
    description: String,
    state: State,
    tags: Option<HashSet<String>>,
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
            let mut tags = HashSet::new();
            tags.insert(slug(tag.into()));

            self.tags = Some(tags);
        } else {
            self.tags.as_mut().unwrap().insert(slug(tag.into()));
        }

        self
    }

    pub fn tags(&mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.tags = Some(tags.into_iter().map(|tag| slug(tag.into())).collect());
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
                tags: HashSet::new(),
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
                tags: HashSet::new(),
                project: "Inbox".to_string()
            }
        );
    }

    #[test]
    fn task_builder_change_tag_works() {
        let task = Task::create("This is a test").tag("Test").build();
        let mut hash_set = HashSet::new();
        hash_set.insert("test".to_string());

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: hash_set,
                project: "Inbox".to_string()
            }
        );
    }

    #[test]
    fn task_builder_change_tags_works() {
        let task = Task::create("This is a test")
            .tags(vec!["Test 1", "Test 2"])
            .build();

        let mut hash_set = HashSet::new();
        hash_set.insert("test-1".to_string());
        hash_set.insert("test-2".to_string());

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: hash_set,
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
                tags: HashSet::new(),
                project: "Testing".to_string()
            }
        );
    }

    #[test]
    fn add_tag_works() {
        let mut task = Task::create("This is a test").project("Testing").build();
        task.add_tag("Testing tags");

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: HashSet::from(["testing-tags".to_string()]),
                project: "Testing".to_string()
            }
        );
    }

    #[test]
    fn add_tags_works() {
        let mut task = Task::create("This is a test").project("Testing").build();
        task.add_tags(["Testing tags", "another tag", "Yet Another Tag"]);

        assert_eq!(
            task,
            Task {
                description: "This is a test".to_string(),
                state: State::ToDo,
                tags: HashSet::from([
                    "testing-tags".to_string(),
                    "another-tag".to_string(),
                    "yet-another-tag".to_string()
                ]),
                project: "Testing".to_string()
            }
        );
    }
}
