use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::{GRID_FOUR, SQUARE_SPLIT_HORIZONTAL, SQUARE_SPLIT_VERTICAL, TABS};
use egui_tiles::ContainerKind;
use typed_builder::TypedBuilder;

/// Grid button widget
#[derive(Debug, TypedBuilder)]
pub struct GridButton<'a> {
    current_value: &'a mut Option<ContainerKind>,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for GridButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text = RichText::new(GRID_FOUR);
        text = if let Some(size) = self.size {
            text.size(size)
        } else {
            text.heading()
        };
        ui.selectable_value(self.current_value, Some(ContainerKind::Grid), text)
            .on_hover_localized("Grid")
    }
}

/// Horizontal button widget
#[derive(Debug, TypedBuilder)]
pub struct HorizontalButton<'a> {
    current_value: &'a mut Option<ContainerKind>,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for HorizontalButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text = RichText::new(SQUARE_SPLIT_HORIZONTAL);
        text = if let Some(size) = self.size {
            text.size(size)
        } else {
            text.heading()
        };
        ui.selectable_value(self.current_value, Some(ContainerKind::Horizontal), text)
            .on_hover_localized("Horizontal")
    }
}

/// Tabs button widget
#[derive(Debug, TypedBuilder)]
pub struct TabsButton<'a> {
    current_value: &'a mut Option<ContainerKind>,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for TabsButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text = RichText::new(TABS);
        text = if let Some(size) = self.size {
            text.size(size)
        } else {
            text.heading()
        };
        ui.selectable_value(self.current_value, Some(ContainerKind::Tabs), text)
            .on_hover_localized("Tabs")
    }
}

/// Vertical button widget
#[derive(Debug, TypedBuilder)]
pub struct VerticalButton<'a> {
    current_value: &'a mut Option<ContainerKind>,
    #[builder(default, setter(strip_option))]
    size: Option<f32>,
}

impl Widget for VerticalButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text = RichText::new(SQUARE_SPLIT_VERTICAL);
        text = if let Some(size) = self.size {
            text.size(size)
        } else {
            text.heading()
        };
        ui.selectable_value(self.current_value, Some(ContainerKind::Vertical), text)
            .on_hover_localized("Vertical")
    }
}
