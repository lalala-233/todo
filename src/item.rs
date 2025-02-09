use eframe::egui::{self, Ui};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ItemType {
    name: String,
    pub states: Vec<String>,
    pub state_counts: HashMap<String, u32>,
}

impl ItemType {
    pub fn show(&mut self, ui: &mut Ui, new_state: &mut String) {
        ui.collapsing(self.name().to_owned(), |ui| {
            self.ui_add_state(ui, new_state);
            self.ui_show_state(ui);
        });
    }
    fn ui_add_state(&mut self, ui: &mut Ui, new_state: &mut String) {
        ui.horizontal(|ui| {
            ui.label("Add state:");
            ui.text_edit_singleline(new_state);
            if ui.button("Add").clicked() {
                let state = new_state.trim().to_string();
                if !state.is_empty() && !self.states.contains(&state) {
                    self.states.push(state.clone());
                    self.state_counts.insert(state, 0);
                    new_state.clear();
                }
            }
        });
    }
    fn ui_show_state(&mut self, ui: &mut Ui) {
        ui.label("State counts:");
        for state in &self.states {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", state));
                let count = self.state_counts.entry(state.clone()).or_insert(0);
                ui.add(egui::DragValue::new(count).range(0..=9999));
            });
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
