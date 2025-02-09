use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Action {
    pub name: String,
    pub item_type: String,
    pub from_state: String,
    pub to_state: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct NewAction {
    pub name: String,
    pub item_type: String,
    pub from_state: String,
    pub to_state: String,
}
