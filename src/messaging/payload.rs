use serde::{Deserialize, Serialize};

// In messaging/payload.rs
#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub timestamp: String,
    pub action: String,
    pub text: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
}

impl MessagePayload {
    pub fn get_parameter_value(&self, name: &str) -> Option<String> {
        self.parameters.iter()
            .find(|p| p.name == name)
            .map(|p| p.value.clone())
    }
}
