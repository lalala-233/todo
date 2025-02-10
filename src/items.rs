use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Items {
    items: Vec<Item>,
}
impl Items {
    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }
}
