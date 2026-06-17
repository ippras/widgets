use crate::r#const::{PERCENT, PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use serde_with::serde_as;
use typed_builder::TypedBuilder;

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision and significant
#[derive(Clone, Copy, Debug, Hash, PartialEq, TypedBuilder)]
#[serde_as]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrecisionAndSignificant<const N: usize> {
    #[builder(default = 1, setter(skip))]
    pub precision: usize,
    #[builder(default, setter(skip))]
    pub significant: bool,

    #[builder(default = [0; N])]
    #[serde_as(as = "[_; N]")]
    pub bookmarks: [usize; N],
}

impl<const N: usize> PrecisionAndSignificant<N> {
    pub fn new() -> Self {
        Self {
            precision: 1,
            significant: false,
            bookmarks: [0; N],
        }
    }
}

impl<const N: usize> PrecisionAndSignificant<N> {
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
                    for bookmark in self.bookmarks {
                        if ui.button(bookmark.to_string()).clicked() {
                            self.precision = bookmark;
                        }
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
