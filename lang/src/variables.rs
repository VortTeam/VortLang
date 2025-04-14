use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum VariableValue {
    String(String),
    Number(f64),
}

#[derive(Debug, Default)]
pub struct VariableStore {
    variables: HashMap<String, VariableValue>,
}

impl VariableStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, value: VariableValue) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&VariableValue> {
        self.variables.get(name)
    }
}