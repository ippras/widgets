use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::SIGMA;
use typed_builder::TypedBuilder;

/// Sum button widget
#[derive(Debug, TypedBuilder)]
pub struct SumButton<'a> {
    atom: &'a str,
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    hover: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for SumButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = (RichText::new(SIGMA), RichText::new(ui.localize(self.atom)));
        atoms = if let Some(size) = self.size {
            (atoms.0.size(size), atoms.1.size(size))
        } else {
            (atoms.0.heading(), atoms.1.heading())
        };
        let mut response = ui.toggle_value(self.selected, atoms);
        if let Some(hover) = self.hover {
            response = response.on_hover_localized(hover)
        }
        response
    }
}
