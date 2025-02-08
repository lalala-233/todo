use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StateManagementApp {
    pub item_types: Vec<ItemType>,
    pub transitions: Vec<Transition>,
    pub new_state: String,
    pub new_item_type_name: String,
    pub new_transition: NewTransition,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ItemType {
    pub name: String,
    pub states: Vec<String>,
    pub state_counts: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Transition {
    pub name: String,
    pub item_type: String,
    pub from_state: String,
    pub to_state: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct NewTransition {
    pub name: String,
    pub item_type: String,
    pub from_state: String,
    pub to_state: String,
}
