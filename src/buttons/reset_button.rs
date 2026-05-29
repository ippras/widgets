use crate::r#const::{RESET, WIDGETS};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::ARROWS_CLOCKWISE;
use typed_builder::TypedBuilder;

/// Reset button widget
#[derive(Debug, TypedBuilder)]
pub struct ResetButton<'a> {
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for ResetButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = RichText::new(ARROWS_CLOCKWISE);
        atoms = if let Some(size) = self.size {
            atoms.size(size)
        } else {
            atoms.heading()
        };
        ui.toggle_value(self.selected, atoms)
            .on_hover_localized(formatcp!("{WIDGETS}_{RESET}"))
    }
}
