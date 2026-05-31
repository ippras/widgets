use crate::r#const::{FILTER, PREFIX as WIDGETS, SORT, ZERO};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::UiExt;
use serde::{Deserialize, Serialize};

/// Zero
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Zero {
    pub checked: bool,
    pub sort: bool,
    pub filter: bool,
}

impl Zero {
    pub fn new() -> Self {
        Self {
            checked: false,
            sort: false,
            filter: false,
        }
    }
}

impl Zero {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{WIDGETS}_{ZERO}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{WIDGETS}_{ZERO}.hover")));
                });
            ui.checkbox(&mut self.checked, ());
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{WIDGETS}_{SORT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{WIDGETS}_{SORT}.hover")));
                });
            ui.checkbox(&mut self.sort, ());
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{WIDGETS}_{FILTER}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{WIDGETS}_{FILTER}.hover")));
                });
            ui.checkbox(&mut self.filter, ());
        });
    }
}
