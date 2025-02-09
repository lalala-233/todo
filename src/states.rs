use crate::*;
use eframe::egui::Ui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct States(Vec<State>);

impl States {
    fn iter(&self) -> impl Iterator<Item = &State> {
        self.0.iter()
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
    pub fn show_add_state(&self, ui: &mut Ui, name: &mut String) {
        ui.label("添加状态：");
        ui.text_edit_singleline(name);
        ui.horizontal(|ui| {
            if ui.button("清空").clicked() {
                name.clear();
            }
            if ui.button("添加").clicked() {
                let state = name.trim().to_string();
                if !state.is_empty() {
                    todo!();
                    // self.states.contains(&state);
                    // self.states.push(state.clone());
                    // self.state_counts.insert(state, 0);
                    name.clear();
                }
            }
        });
    }
}
