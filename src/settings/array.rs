use crate::{
    r#const::{CHECK_ALL, PREFIX, SORT, UNCHECK_ALL},
    utils::format_list_truncated,
};
use const_format::formatcp;
use egui::{ComboBox, Popup, PopupCloseBehavior, RichText, Ui};
use egui_dnd::dnd;
use egui_l10n::prelude::*;
use egui_phosphor::regular::{DOTS_SIX_VERTICAL, FUNNEL, FUNNEL_X, SORT_ASCENDING};
use serde::{Deserialize, Serialize};
use std::slice::Iter;

/// Array
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Array(Vec<Item>);

impl Array {
    pub fn new(items: Vec<Item>) -> Self {
        Self(items)
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let selected_text = format_list_truncated(
            self.0
                .iter()
                .filter_map(|item| item.visible.then_some(ui.localize(&item.name))),
        );
        let response = ComboBox::from_id_salt(ui.next_auto_id())
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .selected_text(&selected_text)
            .show_ui(ui, |ui| self.content(ui))
            .response;
        Popup::context_menu(&response)
            .id(ui.next_auto_id().with("ContextMenu"))
            .show(|ui| {
                if ui
                    .button((FUNNEL, ui.localize(formatcp!("{PREFIX}_{CHECK_ALL}"))))
                    .on_hover_localized(formatcp!("{PREFIX}_{CHECK_ALL}.hover"))
                    .clicked()
                {
                    for item in &mut self.0 {
                        item.visible = true;
                    }
                }
                if ui
                    .button((FUNNEL_X, ui.localize(formatcp!("{PREFIX}_{UNCHECK_ALL}"))))
                    .on_hover_localized(formatcp!("{PREFIX}_{UNCHECK_ALL}.hover"))
                    .clicked()
                {
                    for item in &mut self.0 {
                        item.visible = false;
                    }
                }
                if ui
                    .button((SORT_ASCENDING, ui.localize(formatcp!("{PREFIX}_{SORT}"))))
                    .on_hover_localized(formatcp!("{PREFIX}_{SORT}.hover"))
                    .clicked()
                {
                    self.0.sort_by_key(|item| item.index);
                }
            });
    }

    fn content(&mut self, ui: &mut Ui) {
        let response = dnd(ui, ui.auto_id_with("Array")).show(
            self.0.iter_mut(),
            |ui, item, handle, _state| {
                ui.horizontal(|ui| {
                    let visible = item.visible;
                    handle.ui(ui, |ui| {
                        ui.label(DOTS_SIX_VERTICAL);
                    });
                    ui.checkbox(&mut item.visible, "");
                    let mut text = RichText::new(ui.localize(&item.name));
                    if !visible {
                        text = text.weak();
                    }
                    ui.label(text);
                });
            },
        );
        if response.is_drag_finished() {
            response.update_vec(self.0.as_mut_slice());
        }
    }
}

impl Array {
    pub fn iter(&self) -> Iter<'_, Item> {
        self.0.iter()
    }
}

impl<const N: usize> From<[&str; N]> for Array {
    fn from(value: [&str; N]) -> Self {
        Self(
            value
                .into_iter()
                .enumerate()
                .map(|(index, name)| Item::new(index, name))
                .collect(),
        )
    }
}

/// Item
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Item {
    pub index: usize,
    pub name: String,
    pub visible: bool,
}

impl Item {
    fn new(index: usize, name: impl Into<String>) -> Self {
        Self {
            index,
            name: name.into(),
            visible: true,
        }
    }
}
