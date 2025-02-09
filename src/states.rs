use std::time::Instant;

use crate::*;
use eframe::egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct States {
    states: Vec<State>,
    #[serde(skip)]
    instant: Option<Instant>,
}

impl States {
    const ERROR_TIMEOUT_SEC: u64 = 1;
    fn should_show_error(&self) -> bool {
        self.instant
            .is_some_and(|inst| inst.elapsed().as_secs() < Self::ERROR_TIMEOUT_SEC)
    }
    fn get_new_id(&self) -> u32 {
        self.states.last().unwrap_or(&Default::default()).id() + 1
    }
    fn iter(&self) -> impl Iterator<Item = &State> {
        self.states.iter()
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
                self.states.push(state);
            })
            .ok_or(())
    }
    pub fn show_select_state(&self, ui: &mut Ui, current: &mut State) {
        self.iter().for_each(|state| {
            let selected_value = state.clone();
            let text = state.name();
            if ui
                .selectable_label(*current == selected_value, text)
                .clicked()
            {
                *current = state.clone();
            }
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
