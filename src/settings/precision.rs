use crate::r#const::{PERCENT, PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize, TypedBuilder)]
pub struct Precision {
    #[builder(default = 1, setter(skip))]
    pub precision: usize,
    #[builder(default, setter(skip))]
    pub significant: bool,

    #[builder(default, setter(strip_option))]
    pub percent: Option<bool>,
}

impl Precision {
    pub fn new() -> Self {
        Self {
            precision: 1,
            significant: false,

            percent: None,
        }
    }
}

impl Precision {
    pub fn show(&mut self, ui: &mut Ui) {
        // Precision
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{PRECISION}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{PRECISION}.hover")));
                });
            Slider::new(&mut self.precision, 1..=MAX_PRECISION).ui(ui);
            if ui.button((BOOKMARK, "3")).clicked() {
                self.precision = 3;
            };
        });

        // Significant
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SIGNIFICANT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{SIGNIFICANT}.hover")));
                });
            ui.checkbox(&mut self.significant, ());
        });

        // Percent
        if let Some(percent) = &mut self.percent {
            ui.horizontal(|ui| {
                ui.label(ui.localize(formatcp!("{PREFIX}_{PERCENT}")))
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(formatcp!("{PREFIX}_{PERCENT}.hover")));
                    });
                ui.checkbox(percent, ());
            });
        }
    }
}
