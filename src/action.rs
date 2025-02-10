use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Action {
    pub name: String,
    pub item_type: String,
    pub from_state: State,
    pub to_state: State,
}
