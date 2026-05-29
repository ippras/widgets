use crate::settings::Array;
use const_format::formatcp;
use egui::Ui;
use egui_l10n::prelude::*;
use fatty_acid_expressions::r#const::{BIODIESEL, METABOLIC, NUTRITIONAL, PREFIX, SUM, ratio, sum};
use serde::{Deserialize, Serialize};

/// Expressions
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Expressions {
    pub sum: Array,
    pub ratio: Ratio,
}

impl Expressions {
    pub fn new() -> Self {
        Self {
            sum: Array::from(sum::SUMS),
            ratio: Ratio::new(),
        }
    }
}

impl Expressions {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SUM}")))
                .on_hover_localized(formatcp!("{PREFIX}_{SUM}.hover"));
            self.sum.show(ui);
        });
        self.ratio.show(ui);
    }
}

/// Ratio
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Ratio {
    pub biodiesel: Array,
    pub metabolic: Array,
    pub nutritional: Array,
}

impl Ratio {
    fn new() -> Self {
        Self {
            biodiesel: Array::from(ratio::biodiesel::RATIOS),
            metabolic: Array::from(ratio::metabolic::RATIOS),
            nutritional: Array::from(ratio::nutritional::RATIOS),
        }
    }
}

impl Ratio {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{BIODIESEL}")))
                .on_hover_localized(formatcp!("{PREFIX}_{BIODIESEL}.hover"));
            self.biodiesel.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{METABOLIC}")))
                .on_hover_localized(formatcp!("{PREFIX}_{METABOLIC}.hover"));
            self.metabolic.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{NUTRITIONAL}")))
                .on_hover_localized(formatcp!("{PREFIX}_{NUTRITIONAL}.hover"));
            self.nutritional.show(ui);
        });
    }
}
