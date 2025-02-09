use crate::*;
use eframe::egui::{self, Ui};
use eframe::{Frame, Storage};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StateManagementApp {
    pub item_types: Vec<ItemType>,
    pub actions: Vec<Action>,
    pub new_state: State,
    pub new_item_type_name: String,
    pub new_action: Action,
}

impl eframe::App for StateManagementApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.manage_item_types(ui);
            self.manage_actions(ui);
            self.execute_actions(ui);
            self.show_current_state(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl StateManagementApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut style = egui::Style::default();
        style.interaction.tooltip_delay = 0.1;
        cc.egui_ctx.set_style(style);
        crate::font::set_fonts(&cc.egui_ctx);

        // 从持久化存储加载应用状态
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }

    fn manage_item_types(&mut self, ui: &mut Ui) {
        ui.heading("Manage Item Types");

        // 添加新物品类型
        ui.horizontal(|ui| {
            ui.label("New item type:");
            ui.text_edit_singleline(&mut self.new_item_type_name);
            if ui.button("Add").clicked() {
                let name = self.new_item_type_name.trim().to_string();
                if !name.is_empty() && !self.item_types.iter().any(|i| i.name() == name) {
                    self.item_types.push(ItemType::new(name));
                    self.new_item_type_name.clear();
                }
            }
        }); // 管理现有物品类型
        for item in &mut self.item_types {
            item.show(ui, &mut self.new_state);
        }
    }

    fn manage_actions(&mut self, ui: &mut Ui) {
        ui.heading("Manage actions");

        // 过渡规则表单
        ui.horizontal(|ui| {
            ui.label("Task name:");
            ui.text_edit_singleline(&mut self.new_action.name);
        });

        // 物品类型选择
        egui::ComboBox::from_label("Item Type")
            .selected_text(&self.new_action.item_type)
            .show_ui(ui, |ui| {
                for item in &self.item_types {
                    ui.selectable_value(
                        &mut self.new_action.item_type,
                        item.name().to_string(),
                        item.name(),
                    );
                }
            });

        // 状态选择（需要先选择物品类型）
        if let Some(item) = self
            .item_types
            .iter()
            .find(|i| i.name() == self.new_action.item_type)
        {
            egui::ComboBox::from_label("From State")
                .selected_text(&self.new_action.from_state)
                .show_ui(ui, |ui| {
                    // item.show_select_state(ui, &mut self.new_action.from_state)
                });

            egui::ComboBox::from_label("To State")
                .selected_text(&self.new_action.to_state)
                .show_ui(ui, |ui| {
                    // item.show_select_state(ui, &mut self.new_action.to_state)
                });
        }

        if ui.button("Add action").clicked()
            && !self.new_action.name.is_empty()
            && !self.new_action.item_type.is_empty()
            && !self.new_action.from_state.is_empty()
            && !self.new_action.to_state.is_empty()
        {
            self.actions.push(self.new_action.clone());
            self.new_action = Action::default();
        }
    }

    fn execute_actions(&mut self, ui: &mut Ui) {
        ui.heading("Execute actions");
        for action in &self.actions {
            ui.horizontal(|ui| {
                ui.label(&action.name);
                if ui.button("Execute").clicked() {
                    self.item_types
                        .iter_mut()
                        .find(|i| i.name() == action.item_type)
                        .unwrap()
                        .execute(action);
                }
            });
        }
    }

    fn show_current_state(&self, ui: &mut Ui) {
        ui.heading("Current State");
        for item in &self.item_types {
            item.show_counts(ui)
        }
    }
}
