use crate::r#const::{PREFIX, REACTIVE};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::ROCKET;
use typed_builder::TypedBuilder;

/// Reactive button widget
#[derive(Debug, TypedBuilder)]
pub struct ReactiveButton<'a> {
    selected: &'a mut bool,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for ReactiveButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut atoms = RichText::new(ROCKET);
        atoms = if let Some(size) = self.size {
            atoms.size(size)
        } else {
            atoms.heading()
        };
        ui.toggle_value(self.selected, atoms)
            .on_hover_localized(formatcp!("{PREFIX}_{REACTIVE}"))
            .on_hover_localized("Reactive.hover?State=enabled")
            .on_disabled_hover_localized("Reactive.hover?State=disabled")
    }
}
