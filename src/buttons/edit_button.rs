use crate::r#const::{EDIT, PREFIX};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::PENCIL;
use typed_builder::TypedBuilder;

/// Edit button widget
#[derive(Debug, TypedBuilder)]
pub struct EditButton<'a> {
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for EditButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = RichText::new(PENCIL);
        atoms = if let Some(size) = self.size {
            atoms.size(size)
        } else {
            atoms.heading()
        };
        ui.toggle_value(self.selected, atoms)
            .on_hover_localized(formatcp!("{PREFIX}_{EDIT}"))
    }
}
