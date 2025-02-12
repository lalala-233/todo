use crate::*;
use eframe::egui::{CollapsingHeader, Ui};

pub type Item = Entity<States>;

impl Item {
    pub fn execute(&mut self, action: &Action) {
        todo!()
    }
    pub fn show(&mut self, ui: &mut Ui, new_state: &mut String) {
        CollapsingHeader::new(self.name()).show(ui, |ui| {
            self.show_add_entity(ui, new_state);
            self.show_datas(ui);
        });
    }
}
impl Items {
    pub fn show(&mut self, ui: &mut Ui, new_entity_name: &mut String) {
        self.iter_mut()
            .for_each(|entity| entity.show(ui, new_entity_name))
    }
}
