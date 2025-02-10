use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Action {
    name: String,
    item_type: String,
    from_state: State,
    to_state: State,
}
