use crate::settings::Array;
use const_format::formatcp;
use egui::Ui;
use egui_l10n::ContextExt as _;
use fatty_acid_expressions::r#const::{
    RATIO, SUM,
    ratio::{self, BIODIESEL, METABOLIC, NUTRITIONAL},
    sum,
};
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
            ui.label(ui.localize(SUM)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{SUM}.hover")));
            });
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
            ui.label(ui.localize(BIODIESEL)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{BIODIESEL}.hover")));
            });
            self.biodiesel.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(METABOLIC)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{METABOLIC}.hover")));
            });
            self.metabolic.show(ui);
        });
        ui.horizontal(|ui| {
            ui.label(ui.localize(NUTRITIONAL)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{NUTRITIONAL}.hover")));
            });
            self.nutritional.show(ui);
        });
    }
}
