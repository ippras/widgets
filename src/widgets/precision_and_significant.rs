use crate::{
    r#const::{PRECISION, PREFIX, SIGNIFICANT},
    settings::PrecisionAndSignificant,
};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use typed_builder::TypedBuilder;

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision and significant
#[derive(Debug, TypedBuilder)]
pub struct PrecisionAndSignificantWidget<'a> {
    pub precision_and_significant: &'a mut PrecisionAndSignificant,
    #[builder(default)]
    pub bookmarks: &'a [usize],
}

impl PrecisionAndSignificantWidget<'_> {
    pub fn show(&mut self, ui: &mut Ui) {
        // Precision
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{PRECISION}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{PRECISION}.hover")));
                });
            Slider::new(
                &mut self.precision_and_significant.precision,
                1..=MAX_PRECISION,
            )
            .ui(ui);
            if !self.bookmarks.is_empty() {
                ui.menu_button(BOOKMARK, |ui| {
                    for bookmark in self.bookmarks {
                        ui.selectable_value(
                            &mut self.precision_and_significant.precision,
                            *bookmark,
                            bookmark.to_string(),
                        );
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
            ui.checkbox(&mut self.precision_and_significant.significant, ());
        });
    }
}
