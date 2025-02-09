use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ItemType {
    name: String,
    pub states: Vec<String>,
    pub state_counts: HashMap<String, u32>,
}

impl ItemType {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
