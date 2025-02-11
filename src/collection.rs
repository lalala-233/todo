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
    #[serde(skip)]
    error_start_time: Option<Instant>,
}

impl<T: Default + Clone> Collection<T> {
    const ERROR_TIMEOUT_MILLIS: u128 = 618;
    fn should_show_error(&self) -> bool {
        self.error_start_time
            .is_some_and(|inst| inst.elapsed().as_millis() < Self::ERROR_TIMEOUT_MILLIS)
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

impl<T: Default + Clone> Collection<Entity<T>> {
    fn show_selectable_entity(&self, ui: &mut Ui, current: &mut Entity<T>) {
        self.iter().for_each(|entity| {
            if ui
                .selectable_label(current == entity, entity.name())
                .clicked()
            {
                *current = entity.clone();
            }
        });
    }
    pub fn show_names(&self, ui: &mut Ui) {
        self.iter().for_each(|entity| {
            ui.label(entity.name());
        });
    }
    fn not_contain(&self, name: &str) -> bool {
        !self.iter().any(|s| s.name() == name)
    }
    fn try_add(&mut self, name: String) -> Result<(), ()> {
        // 名字存在时返回 Err
        self.not_contain(&name)
            .then(|| {
                self.entities.push(Entity::new(name));
            })
            .ok_or(())
    }
    pub fn show_add_entity(&mut self, ui: &mut Ui, entity_name: &mut String) {
        ui.horizontal(|ui| {
            ui.label("添加：");
            ui.text_edit_singleline(entity_name);
            if ui.button("清空").clicked() {
                entity_name.clear();
            }
            if ui.button("添加").clicked() {
                let trim_name = entity_name.trim().to_string();
                if !trim_name.is_empty() {
                    match self.try_add(trim_name) {
                        Ok(_) => entity_name.clear(),
                        Err(_) => self.error_start_time = Some(Instant::now()),
                    }
                }
            }
            if self.should_show_error() {
                ui.colored_label(Color32::RED, "状态已存在");
            }
        });
    }
}
impl<T: Default + Clone + Display> Collection<Entity<T>> {
    pub fn show_datas(&self, ui: &mut Ui) {
        self.iter().for_each(|entity| {
            ui.label(format!("{}: {}", entity.name(), **entity)); // as_deref()
        });
    }
}
impl<T: Default + Clone + Display> Collection<Entity<Collection<Entity<T>>>> {
    pub fn show(&mut self, ui: &mut Ui, new_entity_name: &mut String) {
        self.iter_mut()
            .for_each(|entity| entity.show(ui, new_entity_name))
    }
}

pub type States = Collection<State>;
pub type Items = Collection<Item>; //Collection<Entity<Collection<Entity<State>>>>;

// pub type Actions = Collection<Action>;
