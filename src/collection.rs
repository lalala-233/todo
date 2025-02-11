use crate::*;
use eframe::egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::Instant};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Collection<T: Default + Clone> {
    entities: Vec<Entity<T>>,
    #[serde(skip)]
    instant: Option<Instant>,
}

impl<T: Default + Clone> Collection<T> {
    const ERROR_TIMEOUT_MILLIS: u128 = 618;
    fn should_show_error(&self) -> bool {
        self.instant
            .is_some_and(|inst| inst.elapsed().as_millis() < Self::ERROR_TIMEOUT_MILLIS)
    }
    fn iter(&self) -> impl Iterator<Item = &Entity<T>> {
        self.entities.iter()
    }
    fn not_contain(&self, name: &str) -> bool {
        !self.iter().any(|s| s.name() == name)
    }
    fn try_add(&mut self, name: String) -> Result<(), ()> {
        // 名字存在时返回 Err
        self.not_contain(&name)
            .then(|| {
                let entity = Entity::new(name);
                self.entities.push(entity);
            })
            .ok_or(())
    }
    pub fn show_selectable_entity(&self, ui: &mut Ui, current: &mut Entity<T>) {
        self.iter().for_each(|entity| {
            if ui
                .selectable_label(current == entity, entity.name())
                .clicked()
            {
                *current = entity.clone();
            }
        });
    }
    pub fn show_add_entity(&mut self, ui: &mut Ui, entity_name: &mut String) {
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
                    Err(_) => self.instant = Some(Instant::now()),
                }
            }
        }
        if self.should_show_error() {
            ui.colored_label(Color32::RED, "状态已存在");
        }
    }
}
impl<T: Default + Clone + Display> Collection<T> {
    pub fn show_datas(&self, ui: &mut Ui) {
        self.iter().for_each(|entity| {
            ui.label(format!("{}: {}", entity.name(), **entity)); // as_deref()
        });
    }
}
pub type States = Collection<u32>;
pub type Items = Collection<Item>;

// pub type Actions = Collection<Action>;
