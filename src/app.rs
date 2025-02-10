use crate::*;
use eframe::egui::{self, Ui};
use eframe::{Frame, Storage};
use serde::{Deserialize, Serialize};
use std::time::Instant;
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct StateManagementApp {
    pub item_types: Vec<ItemType>,
    pub actions: Vec<Action>,
    pub new_state_name: String,
    pub new_item_type_name: String,
    pub new_action: Action,
    #[serde(skip)]
    pub error_start_time: Option<Instant>,
}

impl eframe::App for StateManagementApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.manage_item_types(ui);
            self.manage_actions(ui);
            self.execute_actions(ui);
            self.manage_state(ui);
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
        ui.heading("类型管理");
        ui.label("新类型：");
        ui.text_edit_singleline(&mut self.new_item_type_name);
        if ui.button("添加").clicked() {
            let name = self.new_item_type_name.trim().to_string();
            if !name.is_empty() && !self.item_types.iter().any(|i| i.name() == name) {
                self.item_types.push(ItemType::new(name));
                self.new_item_type_name.clear();
            }
        }
    }

    fn manage_actions(&mut self, ui: &mut Ui) {
        ui.heading("任务管理");
        // // 过渡规则表单
        // ui.horizontal(|ui| {
        //     ui.label("任务名称");
        //     ui.text_edit_singleline(&mut self.new_action.name);
        // });

        // // 物品类型选择
        // egui::ComboBox::from_label("物品类型")
        //     .selected_text(&self.new_action.item_type)
        //     .show_ui(ui, |ui| {
        //         for item in &self.item_types {
        //             ui.selectable_value(
        //                 &mut self.new_action.item_type,
        //                 item.name().to_string(),
        //                 item.name(),
        //             );
        //         }
        //     });

        // // 状态选择（需要先选择物品类型）
        // if let Some(item) = self
        //     .item_types
        //     .iter()
        //     .find(|i| i.name() == self.new_action.item_type)
        // {
        //     egui::ComboBox::from_label("From State")
        //         .selected_text(&self.new_action.from_state)
        //         .show_ui(ui, |ui| {
        //             item.show_select_state(ui, &mut self.new_action.from_state)
        //         });
        //     egui::ComboBox::from_label("To State")
        //         .selected_text(&self.new_action.to_state)
        //         .show_ui(ui, |ui| {
        //             item.show_select_state(ui, &mut self.new_action.to_state)
        //         });
        // }

        // if ui.button("添加").clicked() && self.new_action.is_empty() {
        //     self.actions.push(self.new_action.clone());
        //     self.new_action = Action::default();
        // }
    }

    fn execute_actions(&mut self, ui: &mut Ui) {
        ui.heading("Execute actions");
        for action in &self.actions {
            ui.horizontal(|ui| {
                todo!();
                // ui.label(&action.name);
                // if ui.button("Execute").clicked() {
                // self.item_types
                //     .iter_mut()
                //     .find(|i| i.name() == action.item_type)
                //     .unwrap()
                //     .execute(action);
                // }
            });
        }
    }

    fn manage_state(&mut self, ui: &mut Ui) {
        ui.heading("Current State");
        for item in &mut self.item_types {
            item.show(ui, &mut self.new_state_name)
        }
    }
}
