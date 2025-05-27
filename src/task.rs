//! Task module

use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

/// A simple task struct that represents a task with an ID, name, and completion status.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Task {
    id: uuid::Uuid,
    name: String,
    completed: bool,
}

#[wasm_bindgen]
impl Task {
    /// Creates a new `Task` with a unique ID and the given name.
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Task {
        Task {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            completed: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) -> Result<(), JsError> {
        match uuid::Uuid::parse_str(&id) {
            Ok(uuid) => {
                self.id = uuid;
                Ok(())
            }
            Err(err) => Err(JsError::new(&format!("Invalid UUID format: {err}"))),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn completed(&self) -> bool {
        self.completed
    }

    #[wasm_bindgen(setter)]
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}

#[wasm_bindgen]
pub struct Tasks(Vec<Task>);

#[wasm_bindgen]
impl Tasks {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(vec![])
    }

    #[wasm_bindgen]
    pub fn list(&self) -> Vec<Task> {
        self.0.clone()
    }

    #[wasm_bindgen]
    pub fn add(&mut self, task: Task) {
        self.0.push(task);
    }

    #[wasm_bindgen]
    pub fn pop(&mut self) -> Option<Task> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    #![cfg(target_arch = "wasm32")]

    use super::*;
    use wasm_bindgen_test::*;

    extern crate wasm_bindgen_test;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_new_task() {
        let task = Task::new("Test Task");
        assert_eq!(task.name(), "Test Task");
        assert!(!task.completed());
    }

    #[wasm_bindgen_test]
    fn test_set_task_id() {
        let mut task = Task::new("Test Task");
        let new_id = uuid::Uuid::new_v4().to_string();
        task.set_id(new_id.clone()).unwrap();
        assert_eq!(task.id(), new_id);
    }

    #[wasm_bindgen_test]
    fn test_set_task_id_invalid_uuid() {
        let mut task = Task::new("Test Task");
        let result = task.set_id("12454".to_string());
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_new_tasks() {
        let tasks = Tasks::new();
        assert!(tasks.list().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_tasks_add() {
        let mut tasks = Tasks::new();
        let task = Task::new("My task");
        tasks.add(task);

        assert_eq!(tasks.list().len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_tasks_pop() {
        let mut tasks = Tasks::new();
        let task = Task::new("My task");
        tasks.add(task);

        assert_eq!(tasks.list().len(), 1);

        tasks.pop();
        assert!(tasks.list().is_empty());
    }
}
