use crate::*;
use eframe::egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
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
    const ERROR_DISPLAY_DURATION: u64 = 618; // 618ms对应约黄金分割比例
    fn should_show_error(&self) -> bool {
        self.error_start_time()
            .is_some_and(|inst| (inst.elapsed().as_millis() as u64) < Self::ERROR_DISPLAY_DURATION)
    }
    fn get_created_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 // 除非用户手动调整时间至 UNIX_EPOCH 以前，u64 表示到 5 亿年后
    }
    fn change_selected_state(&mut self) {
        self.is_selected = !self.is_selected
    }
    fn error_start_time(&self) -> &Option<Instant> {
        &self.error_start_time
    }
    fn is_selected(&self) -> bool {
        self.is_selected
    }
    fn update_error_start(&mut self) {
        self.error_start_time = Some(Instant::now())
    }
    fn created_time(&self) -> u64 {
        self.created_time
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    fn new(name: String) -> Self {
        Self {
            name,
            created_time: Self::get_created_time(),
            ..Default::default()
        }
    }
    // 相当于 deref()
    fn data(&self) -> &T {
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
pub type Collection<T> = Entity<Vec<T>>;
type Entities<T> = Collection<Entity<T>>; // Entity<Vec<Entity>>，如 Item = Entities<State>
impl<T: Default + Clone> Collection<T> {
    fn try_show_error(&self, ui: &mut Ui) {
        if self.should_show_error() {
            ui.colored_label(Color32::RED, "状态已存在");
        }
    }
}

impl<T: Default + Clone> Entities<T> {
    fn show_selectable_entity(&mut self, ui: &mut Ui) {
        for entity in self.iter_mut() {
            if ui
                .selectable_label(entity.is_selected(), entity.name())
                .clicked()
            {
                entity.change_selected_state();
            }
        }
    }
    pub fn show_delete(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.show_selectable_entity(ui);
            if ui.button("删除").clicked() {
                self.delete_selected()
            }
        });
    }
    fn not_contain(&self, name: &str) -> bool {
        !self.iter().any(|s| s.name() == name)
    }
    fn delete_selected(&mut self) {
        self.retain(|entity| !entity.is_selected()) // 保留未选中
    }
    fn try_add(&mut self, name: &mut String) {
        let trim_name = name.trim().to_string();
        if !trim_name.is_empty() && self.not_contain(&trim_name) {
            self.push(Entity::new(trim_name));
            name.clear();
        } else {
            self.update_error_start();
        };
    }
    pub fn show_add(&mut self, ui: &mut Ui, entity_name: &mut String) {
        ui.horizontal(|ui| {
            ui.label("添加：");
            ui.text_edit_singleline(entity_name);
            if ui.button("清空").clicked() {
                entity_name.clear();
            }
            if ui.button("添加").clicked() {
                self.try_add(entity_name);
            }
            self.try_show_error(ui);
        });
    }
}
impl<T: Default + Clone + Display> Entities<T> {
    pub fn show_datas(&self, ui: &mut Ui) {
        self.iter().for_each(|entity| {
            ui.label(format!("{}: {}", entity.name(), entity.data()));
        });
    }
}
pub type States = Collection<State>;
pub type Items = Collection<Item>;
