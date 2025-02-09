use crate::*;
use eframe::egui::{self, CollapsingHeader, Ui};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
                // for state in self.states() {
                //     ui.label(format!("{}: {}", state, self.get_counts(state)));
                // }
            });
    }
    fn get_counts(&self, state: &State) -> u32 {
        self.states.value(state).unwrap() // 不会 panic
    }
    pub fn show(&mut self, ui: &mut Ui, new_state: &mut State) {
        ui.collapsing(self.name().to_owned(), |ui| {
            self.ui_add_state(ui, new_state);
            self.ui_show_state(ui);
        });
    }
    fn ui_add_state(&mut self, ui: &mut Ui, new_state: &mut State) {
        ui.horizontal(|ui| {
            todo!();
            // ui.label("Add state:");
            // ui.text_edit_singleline(new_state);
            // if ui.button("Add").clicked() {
            //     let state = new_state.trim().to_string();
            // if !state.is_empty() {
            // self.states.contains(&state);
            // self.states.push(state.clone());
            // self.state_counts.insert(state, 0);
            // new_state.clear();
            // }
            // }
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
