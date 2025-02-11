use crate::*;
use eframe::egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Collection<T: Entity + Clone> {
    entitys: Vec<T>,
    #[serde(skip)]
    instant: Option<Instant>,
}

fn get_created_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() // 除非用户手动调整时间至 UNIX_EPOCH 以前
}
impl<T: Entity + Clone> Collection<T> {
    const ERROR_TIMEOUT_MILLIS: u128 = 618;
    fn should_show_error(&self) -> bool {
        self.instant
            .is_some_and(|inst| inst.elapsed().as_millis() < Self::ERROR_TIMEOUT_MILLIS)
    }
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.entitys.iter()
    }
    fn not_contain(&self, name: &str) -> bool {
        !self.iter().any(|s| s.name() == name)
    }
    fn try_add(&mut self, name: String) -> Result<(), ()> {
        // 名字存在时返回 Err
        self.not_contain(&name)
            .then(|| {
                let entity = T::new(name, get_created_time());
                self.entitys.push(entity);
            })
            .ok_or(())
    }
    pub fn show_selectable_entity(&self, ui: &mut Ui, current: &mut T) {
        self.iter().for_each(|entity| {
            if ui
                .selectable_label(current.eq(entity), entity.name())
                .clicked()
            {
                *current = entity.clone();
            }
        });
    }
    pub fn show_datas(&self, ui: &mut Ui) {
        self.iter().for_each(|entity| {
            ui.label(format!("{}: {}", entity.name(), entity.data()));
        });
    }
    pub fn show_add_entity(&mut self, ui: &mut Ui, entity_name: &mut String) {
        ui.label("添加：");
        ui.text_edit_singleline(entity_name);
        ui.horizontal(|ui| {
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
        });
    }
}

pub type States = Collection<State>;
pub type Items = Collection<Item>;
pub type Actions = Collection<Action>;
