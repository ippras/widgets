use crate::{
    Show,
    r#const::{FILTER, HIGHLIGHT, PREFIX, SORT},
};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::ContextExt as _;

/// Highlight, sort, filter
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            ui.label(ui.localize(formatcp!("{PREFIX}_{HIGHLIGHT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{HIGHLIGHT}.hover")));
                });
            ui.checkbox(&mut self.highlight, ());
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SORT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{SORT}.hover")));
                });
            ui.checkbox(&mut self.sort, ());
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{FILTER}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{FILTER}.hover")));
                });
            ui.checkbox(&mut self.filter, ());
        });
    }
}

impl Default for HighlightSortFilter {
    fn default() -> Self {
        Self::new()
    }
}
