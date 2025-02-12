use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Entity<T: Default + Clone> {
    name: String,
    created_time: u128,
    data: T,
}
impl<T: Default + Clone> Entity<T> {
    fn get_created_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() // 除非用户手动调整时间至 UNIX_EPOCH 以前
    }
    pub fn created_time(&self) -> u128 {
        self.created_time
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            created_time: Self::get_created_time(),
            data: Default::default(),
        }
    }
}
impl<T: Default + Clone> PartialEq for Entity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.created_time == other.created_time
    }
}
impl<T: Default + Clone> Deref for Entity<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T: Default + Clone> DerefMut for Entity<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
pub type State = Entity<u32>;
