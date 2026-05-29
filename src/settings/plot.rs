use crate::r#const::{DRAG, LEGEND, PREFIX, RADIUS_OF_POINTS, SCROLL};
use const_format::formatcp;
use egui::{Slider, Ui, Vec2b, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Plot
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Plot {
    pub drag: Vec2b,
    pub legend: bool,
    pub radius_of_points: u8,
    pub scroll: Vec2b,
}

impl Plot {
    pub fn new() -> Self {
        Self {
            drag: Vec2b::TRUE,
            legend: true,
            radius_of_points: 2,
            scroll: Vec2b::TRUE,
        }
    }
}

impl Plot {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SCROLL}")));
            ui.checkbox(&mut self.scroll.x, "x");
            ui.checkbox(&mut self.scroll.y, "y");
        });

        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{DRAG}")));
            ui.checkbox(&mut self.drag.x, "x");
            ui.checkbox(&mut self.drag.y, "y");
        });

        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{LEGEND}")));
            ui.checkbox(&mut self.legend, "");
        });

        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{RADIUS_OF_POINTS}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{RADIUS_OF_POINTS}.hover")));
                });
            Slider::new(&mut self.radius_of_points, 0..=u8::MAX)
                .logarithmic(true)
                .ui(ui);
            if ui.button((BOOKMARK, "2")).clicked() {
                self.radius_of_points = 2;
            };
        });
    }
}

impl Hash for Plot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.drag.x.hash(state);
        self.drag.y.hash(state);
        self.legend.hash(state);
        self.radius_of_points.hash(state);
        self.scroll.x.hash(state);
        self.scroll.y.hash(state);
    }
}
