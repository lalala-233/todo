use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};
pub trait Entity {
    type Data: Display;
    fn created_time(&self) -> u128;
    fn name(&self) -> &str;
    fn data(&self) -> Self::Data;
    fn new(name: String, created_time: u128) -> Self;
    fn eq(&self, other: &Self) -> bool {
        self.created_time() == other.created_time()
    }
}
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct MyEntity<T: Default + Clone + Display> {
    name: String,
    created_time: u128,
    data: T,
}
impl<T: Default + Clone + Display> MyEntity<T> {
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
    pub fn data(&self) -> &T {
        &self.data
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            created_time: Self::get_created_time(),
            data: Default::default(),
        }
    }
}
impl<T: Default + Clone + Display> PartialEq for MyEntity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.created_time == other.created_time
    }
}
