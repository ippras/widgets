use crate::r#const::{PERCENT, PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Percent
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Percent {
    checked: bool,
}

impl Deref for Percent {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.checked
    }
}

impl DerefMut for Percent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.checked
    }
}

impl Percent {
    pub fn new() -> Self {
        Self { checked: true }
    }
}

impl Percent {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{PERCENT}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{PERCENT}.hover")));
                });
            ui.checkbox(&mut self.checked, ());
        });
    }
}
