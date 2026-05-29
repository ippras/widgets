use crate::r#const::{PREFIX, SETTINGS};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::SLIDERS_HORIZONTAL;
use typed_builder::TypedBuilder;

/// Settings button widget
#[derive(Debug, TypedBuilder)]
pub struct SettingsButton<'a> {
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for SettingsButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = RichText::new(SLIDERS_HORIZONTAL);
        atoms = if let Some(size) = self.size {
            atoms.size(size)
        } else {
            atoms.heading()
        };
        ui.toggle_value(self.selected, atoms).on_hover_ui(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SETTINGS}")));
        })
    }
}
