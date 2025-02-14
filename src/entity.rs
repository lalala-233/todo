use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Entity<T: Default + Clone> {
    name: String,
    created_time: u64,
    data: T,
    is_selected: bool,
    #[serde(skip)]
    error_start_time: Option<Instant>,
}
impl<T: Default + Clone> Entity<T> {
    fn get_created_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 // 除非用户手动调整时间至 UNIX_EPOCH 以前，u64 表示到 5 亿年后
    }
    pub fn change_selected_state(&mut self) {
        self.is_selected = !self.is_selected
    }
    pub fn error_start_time(&self) -> &Option<Instant> {
        &self.error_start_time
    }
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }
    pub fn update_error_start(&mut self) {
        self.error_start_time = Some(Instant::now())
    }
    pub fn created_time(&self) -> u64 {
        self.created_time
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            created_time: Self::get_created_time(),
            ..Default::default()
        }
    }
    // 相当于 deref()
    pub fn data(&self) -> &T {
        &self.data
    }
}
impl<T: Default + Clone> PartialEq for Entity<T> {
    // 采用简化实现，需要确保 created_time 唯一
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
