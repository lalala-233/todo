use crate::*;
use eframe::egui::{CollapsingHeader, Ui};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ItemType {
    name: String,
    states: States,
}

impl ItemType {
    pub fn execute(&mut self, _action: &Action) {
        todo!()
    }
    pub fn show_select_state(&self, ui: &mut Ui, current: &mut State) {
        self.states.selectable_value(ui, current);
    }
    pub fn show_counts(&self, ui: &mut Ui) {
        CollapsingHeader::new(self.name())
            .id_salt(self.name().to_uppercase()) // 为了防止和 show() 出现冲突
            .show(ui, |ui| {
                self.states.show_counts(ui);
            });
    }
    fn get_counts(&self, state: &State) -> u32 {
        self.states.value(state).unwrap() // 不会 panic
    }
    pub fn show(&mut self, ui: &mut Ui, new_state: &mut String) {
        ui.collapsing(self.name().to_owned(), |ui| {
            self.ui_add_state(ui, new_state);
            self.ui_show_state(ui);
        });
    }
    fn ui_add_state(&mut self, ui: &mut Ui, name: &mut String) {
        ui.horizontal(|ui| {
            self.states.show_add_state(ui, name);
        });
    }
    fn ui_show_state(&mut self, ui: &mut Ui) {
        ui.label("State counts:");
        todo!()
        // for state in &self.states {
        //     ui.horizontal(|ui| {
        //         ui.label(format!("{}:", state));
        //         let count = self.state_counts.entry(state.clone()).or_insert(0);
        //         ui.add(egui::DragValue::new(count).range(0..=9999));
        //     });
        // }
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
