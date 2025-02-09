use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct State {
    id: u32,
    name: String,
    count: u32,
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl State {
    pub fn new(name: String, id: u32) -> Self {
        Self { id, name, count: 0 }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}
