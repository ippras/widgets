use crate::r#const::RESIZABLE;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::{l10n, prelude::*};
use egui_phosphor::regular::ARROWS_HORIZONTAL;
use typed_builder::TypedBuilder;

/// Resizable button widget
#[derive(Debug, TypedBuilder)]
pub struct ResizableButton<'a> {
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for ResizableButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = RichText::new(ARROWS_HORIZONTAL);
        atoms = if let Some(size) = self.size {
            atoms.size(size)
        } else {
            atoms.heading()
        };
        ui.toggle_value(self.selected, atoms)
            .on_hover_localized(l10n!(RESIZABLE))
    }
}
