use crate::r#const::{PREFIX, RENAME};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::TEXT_AA;
use typed_builder::TypedBuilder;

/// Rename button widget
#[derive(Debug, TypedBuilder)]
pub struct RenameButton<'a> {
    atom: &'a str,
    #[builder(default)]
    heading: bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for RenameButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = (RichText::new(TEXT_AA), RichText::new(self.atom));
        if self.heading {
            atoms = (atoms.0.heading(), atoms.1.heading());
        }
        if let Some(size) = self.size {
            atoms = (atoms.0.size(size), atoms.1.size(size))
        }
        ui.button(atoms)
            .on_hover_localized(formatcp!("{PREFIX}_{RENAME}"))
    }
}
