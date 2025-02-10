use crate::*;
use eframe::egui::{CollapsingHeader, Ui};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Item {
    name: String,
    states: States,
}

impl Item {
    pub fn execute(&mut self, action: &Action) {
        todo!()
    }
    pub fn show_select_state(&self, ui: &mut Ui, current: &mut State) {
        self.states.show_select_state(ui, current);
    }
    pub fn show(&mut self, ui: &mut Ui, new_state: &mut String) {
        CollapsingHeader::new(self.name()).show(ui, |ui| {
            self.ui_add_state(ui, new_state);
            self.states.show_counts(ui);
        });
    }
    fn ui_add_state(&mut self, ui: &mut Ui, name: &mut String) {
        ui.horizontal(|ui| {
            self.states.show_add_state(ui, name);
        });
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
