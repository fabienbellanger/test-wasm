//! Task module

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Task {
    name: String,
    completed: bool,
}

#[wasm_bindgen]
impl Task {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Task {
        Task {
            name: name.to_string(),
            completed: false,
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
pub fn add_task(name: String) -> Task {
    Task::new(&name)
}
