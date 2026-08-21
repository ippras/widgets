use crate::r#const::{PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use typed_builder::TypedBuilder;

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision and significant
#[derive(Clone, Debug, Hash, PartialEq, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrecisionAndSignificant {
    #[builder(default = 1, setter(skip))]
    pub precision: usize,
    #[builder(default, setter(skip))]
    pub significant: bool,

    #[builder(default, via_mutators, mutators(
        pub fn bookmark(self, value: usize) {
            self.bookmarks.push(value);
        }
    ))]
    pub bookmarks: Vec<usize>,
}

impl PrecisionAndSignificant {
    pub fn new() -> Self {
        Self::builder().build()
    }
}

impl PrecisionAndSignificant {
    pub fn show(&mut self, ui: &mut Ui) {
        // Precision
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{PRECISION}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{PRECISION}.hover")));
                });
            Slider::new(&mut self.precision, 1..=MAX_PRECISION).ui(ui);
            if !self.bookmarks.is_empty() {
                ui.menu_button(BOOKMARK, |ui| {
                    for bookmark in &self.bookmarks {
                        ui.selectable_value(&mut self.precision, *bookmark, bookmark.to_string());
                    }
                });
            }
        });

        // Significant
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SIGNIFICANT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{SIGNIFICANT}.hover")));
                });
            ui.checkbox(&mut self.significant, ());
        });

        // // Percent
        // if let Some(percent) = &mut self.percent {
        //     ui.horizontal(|ui| {
        //         ui.label(ui.localize(formatcp!("{PREFIX}_{PERCENT}")))
        //             .on_hover_ui(|ui| {
        //                 ui.label(ui.localize(formatcp!("{PREFIX}_{PERCENT}.hover")));
        //             });
        //         ui.checkbox(percent, ());
        //     });
        // }
    }
}
