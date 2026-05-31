use crate::{
    Show,
    r#const::{FILTER, PREFIX as WIDGETS, SORT, THRESHOLD},
};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::UiExt;
use serde::{Deserialize, Serialize};

/// Threshold
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Threshold<T: Show> {
    pub threshold: T,
    pub sort: bool,
    pub filter: bool,
}

impl Threshold<bool> {
    pub fn new() -> Self {
        Self {
            threshold: false,
            sort: false,
            filter: false,
        }
    }
}

impl<T: Show> Show for Threshold<T> {
    fn show(&mut self, ui: &mut Ui) {
        self.threshold.show(ui);
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

impl Show for bool {
    fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{WIDGETS}_{THRESHOLD}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{WIDGETS}_{THRESHOLD}.hover")));
                });
            ui.checkbox(self, ());
        });
    }
}
