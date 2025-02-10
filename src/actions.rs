use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Actions {
    actions: Vec<Action>,
}
