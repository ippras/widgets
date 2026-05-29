use crate::r#const::{LEFT_PANEL, WIDGETS};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::SIDEBAR_SIMPLE;
use typed_builder::TypedBuilder;

/// Left panel button widget
#[derive(Debug, TypedBuilder)]
pub struct LeftPanelButton<'a> {
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for LeftPanelButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = RichText::new(SIDEBAR_SIMPLE);
        atoms = if let Some(size) = self.size {
            atoms.size(size)
        } else {
            atoms.heading()
        };
        ui.toggle_value(self.selected, atoms)
            .on_hover_localized(formatcp!("{WIDGETS}_{LEFT_PANEL}"))
    }
}
