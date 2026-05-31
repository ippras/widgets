use crate::{
    Show,
    r#const::{FILTER, HIGHLIGHT, PREFIX as WIDGETS, SORT},
};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::UiExt;
use serde::{Deserialize, Serialize};

/// Highlight, sort, filter
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct HighlightSortFilter {
    pub highlight: bool,
    pub sort: bool,
    pub filter: bool,
}

impl HighlightSortFilter {
    pub fn new() -> Self {
        Self {
            highlight: false,
            sort: false,
            filter: false,
        }
    }
}

impl Show for HighlightSortFilter {
    fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{WIDGETS}_{HIGHLIGHT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{WIDGETS}_{HIGHLIGHT}.hover")));
                });
            ui.checkbox(&mut self.highlight, ());
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
