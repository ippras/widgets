use crate::r#const::{PERCENT, PRECISION, SIGNIFICANT};
use egui::{Slider, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::BOOKMARK;
use serde::{Deserialize, Serialize};

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision and standard deviation
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Precision {
    pub percent: bool,
    pub precision: usize,
    pub significant: bool,
}

impl Precision {
    pub fn new() -> Self {
        Self {
            percent: true,
            precision: 1,
            significant: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        // Percent
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(PERCENT)))
                .on_hover_localized(l10n!(PERCENT; hover));
            ui.checkbox(&mut self.percent, ());
        });

        // Precision
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(PRECISION)))
                .on_hover_localized(l10n!(PRECISION; hover));
            Slider::new(&mut self.precision, 1..=MAX_PRECISION).ui(ui);
            if ui.button((BOOKMARK, "3")).clicked() {
                self.precision = 3;
            };
        });

        // Significant
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(SIGNIFICANT)))
                .on_hover_localized(l10n!(SIGNIFICANT; hover));
            ui.checkbox(&mut self.significant, ());
        });
    }
}
