use crate::{
    Show,
    r#const::{PREFIX as WIDGETS, THRESHOLD, ZERO},
};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::UiExt;
use serde::{Deserialize, Serialize};

/// Threshold zero
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct ThresholdZero {
    pub checked: bool,
}

impl ThresholdZero {
    pub fn new() -> Self {
        Self { checked: false }
    }
}

impl Show for ThresholdZero {
    fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{WIDGETS}_{THRESHOLD}{ZERO}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{WIDGETS}_{THRESHOLD}{ZERO}.hover")));
                });
            ui.checkbox(&mut self.checked, ());
        });
    }
}
