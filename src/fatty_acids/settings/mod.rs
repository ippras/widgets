pub use self::expressions::Expressions;

use crate::r#const::{FILTER, SORT, ZERO};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::UiExt;
use fatty_acid_expressions::r#const::PREFIX as FAE;
use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub zero: bool,
    pub sort: bool,
    pub filter: bool,
    pub expressions: Expressions,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            zero: false,
            sort: false,
            filter: false,
            expressions: Expressions::new(),
        }
    }
}

impl Settings {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{FAE}_{ZERO}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{FAE}_{ZERO}.hover")));
                });
            ui.checkbox(&mut self.zero, ());
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{FAE}_{SORT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{FAE}_{SORT}.hover")));
                });
            ui.checkbox(&mut self.sort, ());
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{FAE}_{FILTER}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{FAE}_{FILTER}.hover")));
                });
            ui.checkbox(&mut self.filter, ());
        });
        ui.separator();
        self.expressions.show(ui);
    }
}

pub mod expressions;
