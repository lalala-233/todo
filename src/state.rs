use eframe::egui::Ui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct State {
    id: u32,
    name: String,
    count: u32,
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl State {
    pub fn new(name: String, id: u32) -> Self {
        Self { id, name, count: 0 }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct States(Vec<State>);

impl States {
    pub fn selectable_value(&self, ui: &mut Ui, current: &mut State) {
        for state in &self.0 {
            ui.selectable_value(current, state.clone(), state.name());
        }
    }
    pub fn value(&self, state: &State) -> Option<u32> {
        self.0.iter().find(|&s| s == state).map(|s| s.count)
    }
}
