use crate::*;
use eframe::egui::{Color32, Ui};
use std::fmt::Display;

// #[derive(Serialize, Deserialize, Default, Clone, Debug)]
// #[serde(default)]
// pub struct Collection<T: Default + Clone> {
//     entities: Vec<T>,
//     selected_value_created_time: u64,
//     #[serde(skip)]
//     error_start_time: Option<Instant>,
// }
pub type Collection<T> = Entity<Vec<T>>;
pub type Entities<T> = Collection<Entity<T>>;// Entity<Vec<Entity>>，如 Item = Entities<State>
impl<T: Default + Clone> Collection<T> {
    const ERROR_DISPLAY_DURATION: u64 = 618; // 618ms对应约黄金分割比例
    fn should_show_error(&self) -> bool {
        self.error_start_time()
            .is_some_and(|inst| (inst.elapsed().as_millis() as u64) < Self::ERROR_DISPLAY_DURATION)
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
    fn try_show_error(&self, ui: &mut Ui) {
        if self.should_show_error() {
            ui.colored_label(Color32::RED, "状态已存在");
        }
    }
    fn delete_selected(&mut self) {
        self.retain(|entity| !entity.is_selected()) // 保留未选中
    }

    pub fn try_add(&mut self, name: &mut String) {
        let trim_name = name.trim().to_string();
        if !trim_name.is_empty() && self.not_contain(&trim_name) {
            self.push(Entity::new(trim_name));
            name.clear();
        } else {
            self.update_error_start();
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
            ui.label(format!("{}: {}", entity.name(), entity.data())); 
        });
    }
}
pub type States = Collection<State>;
pub type Items = Collection<Item>;
// pub type Actions = Collection<Action>;
