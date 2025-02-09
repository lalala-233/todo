use crate::*;
use eframe::egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct States(Vec<State>);

impl States {
    fn get_new_id(&self) -> u32 {
        self.0.last().unwrap_or(&Default::default()).id() + 1
    }
    fn iter(&self) -> impl Iterator<Item = &State> {
        self.0.iter()
    }
    fn not_contain(&self, name: &str) -> bool {
        !self.iter().any(|s| s.name() == name)
    }
    /// 名字存在时返回 Err
    fn try_add_state(&mut self, name: String) -> Result<(), ()> {
        self.not_contain(&name)
            .then(|| {
                let id = self.get_new_id();
                let state = State::new(name, id);
                self.0.push(state);
            })
            .ok_or(())
    }
    pub fn selectable_value(&self, ui: &mut Ui, current: &mut State) {
        self.iter().for_each(|state| {
            ui.selectable_value(current, state.clone(), state.name());
        });
    }
    pub fn value(&self, state: &State) -> Option<u32> {
        self.iter().find(|&s| s == state).map(|s| s.count())
    }
    pub fn show_counts(&self, ui: &mut Ui) {
        self.iter().for_each(|state| {
            ui.label(format!("{}: {}", state.name(), state.count()));
        });
    }
    pub fn show_add_state(&mut self, ui: &mut Ui, state_name: &mut String) {
        ui.label("添加状态：");
        ui.text_edit_singleline(state_name);
        ui.horizontal(|ui| {
            if ui.button("清空").clicked() {
                state_name.clear();
            }
            if ui.button("添加").clicked() {
                let trim_name = state_name.trim().to_string();
                if !trim_name.is_empty() {
                    match self.try_add_state(trim_name) {
                        Ok(_) => state_name.clear(),
                        Err(_) => {
                            ui.colored_label(Color32::RED, "状态已存在");
                        }
                    }
                }
            }
        });
    }
}
