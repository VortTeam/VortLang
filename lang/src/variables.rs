use std::collections::HashMap;
use crate::error::VortError;

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

    pub fn insert(&mut self, name: String, value: VariableValue) -> Result<(), VortError> {
        if let Some(existing) = self.variables.get(&name) {
            match (existing, &value) {
                (VariableValue::String(_), VariableValue::Number(_)) => {
                    return Err(VortError::RuntimeError(
                        format!("Can't change value of a string variable ({}) to a number", name)
                    ));
                }
                (VariableValue::Number(_), VariableValue::String(_)) => {
                    return Err(VortError::RuntimeError(
                        format!("Can't change value of a numerical variable ({}) to a string", name)
                    ));
                }
                _ => {}
            }
        }
        self.variables.insert(name, value);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&VariableValue> {
        self.variables.get(name)
    }
}
