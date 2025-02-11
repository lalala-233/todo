use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct State {
    created_time: u128,
    name: String,
    count: u32,
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.created_time == other.created_time
    }
}

impl Entity for State {
    fn created_time(&self) -> u128 {
        self.created_time
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn new(name: String, created_time: u128) -> Self {
        Self {
            name,
            created_time,
            count: 0,
        }
    }

    type Data = u32;

    fn data(&self) -> Self::Data {
        todo!()
    }
}
impl State {
    pub fn count(&self) -> u32 {
        self.count
    }
}
