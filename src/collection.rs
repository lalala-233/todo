use crate::*;
use eframe::egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    time::Instant,
};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Collection<T: Default + Clone> {
    entities: Vec<T>,
    selected_value_created_time: u64,
    #[serde(skip)]
    error_start_time: Option<Instant>,
}

pub type Entities<T> = Collection<Entity<T>>;
impl<T: Default + Clone> Collection<T> {
    const ERROR_DISPLAY_DURATION: u64 = 618; // 618ms对应约黄金分割比例
    fn should_show_error(&self) -> bool {
        self.error_start_time
            .is_some_and(|inst| (inst.elapsed().as_millis() as u64) < Self::ERROR_DISPLAY_DURATION)
    }
}
impl<T: Default + Clone> Deref for Collection<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}
impl<T: Default + Clone> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entities
    }
}

impl<T: Default + Clone> Entities<T> {
    fn show_selectable_entity(&mut self, ui: &mut Ui) {
        let current = &mut self.selected_value_created_time;
        for entity in &self.entities {
            if ui
                .selectable_label(*current == entity.created_time(), entity.name())
                .clicked()
            {
                *current = entity.created_time();
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
    fn try_show_error(&self, ui: &mut Ui) {
        if self.should_show_error() {
            ui.colored_label(Color32::RED, "状态已存在");
        }
    }
    fn delete_selected(&mut self) {
        if let Some(index) = self
            .iter()
            .position(|s| s.created_time() == self.selected_value_created_time)
        {
            self.remove(index);
            self.selected_value_created_time = 0;
        } else {
            // self.error = Some(Instant::now())
        };
    }

    pub fn try_add(&mut self, name: &mut String) {
        let trim_name = name.trim().to_string();
        if !trim_name.is_empty() && self.not_contain(&trim_name) {
            self.push(Entity::new(trim_name));
            name.clear();
        } else {
            // self.error = Some(Instant::now()) FIXME:
        };
    }
    pub fn show_add_entity(&mut self, ui: &mut Ui, entity_name: &mut String) {
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
            ui.label(format!("{}: {}", entity.name(), **entity)); // as_deref()
        });
    }
}

pub type States = Collection<State>;
pub type Items = Collection<Item>; //Collection<Entity<Collection<Entity<State>>>>;

// pub type Actions = Collection<Action>;
