use crate::r#const::{ABSOLUTE, DELTA_DEGREES_OF_FREEDOM, MEAN, RELATIVE, STANDARD_DEVIATION};
use egui::{ComboBox, Slider, Ui, Widget};
use egui_l10n::prelude::*;
use serde::{Deserialize, Serialize};

/// Mean and standard deviation
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Mean {
    pub mean: bool,
    pub standard_deviation: bool,
    pub kind: Kind,
    pub ddof: u8,
}

impl Mean {
    pub fn new() -> Self {
        Self {
            mean: false,
            standard_deviation: false,
            kind: Kind::Absolute,
            ddof: 1,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(MEAN)))
                .on_hover_localized(l10n!(MEAN; hover));
            ui.checkbox(&mut self.mean, ());
        });

        ui.horizontal(|ui| {
            if !self.mean {
                ui.disable();
            }
            ui.label(ui.localize(l10n!(STANDARD_DEVIATION)))
                .on_hover_localized(l10n!(STANDARD_DEVIATION; hover));
            ui.checkbox(&mut self.standard_deviation, ());
            if !self.standard_deviation {
                ui.disable();
            }
            ComboBox::from_id_salt(ui.next_auto_id())
                .selected_text(self.kind.text())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.kind,
                        Kind::Absolute,
                        l10n!(Kind::Absolute.text()),
                    );
                    ui.selectable_value(
                        &mut self.kind,
                        Kind::Relative,
                        l10n!(Kind::Relative.text()),
                    );
                });
        });

        // [DDOF](https://numpy.org/devdocs/reference/generated/numpy.std.html)
        ui.horizontal(|ui| {
            if !self.mean || !self.standard_deviation {
                ui.disable();
            }
            ui.label(ui.localize(l10n!(DELTA_DEGREES_OF_FREEDOM; abbreviation)))
                .on_hover_localized(l10n!(DELTA_DEGREES_OF_FREEDOM))
                .on_hover_localized(l10n!(DELTA_DEGREES_OF_FREEDOM; hover));
            Slider::new(&mut self.ddof, 0..=1)
                .update_while_editing(false)
                .ui(ui);
        });
    }
}

/// Standard deviation kind
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Kind {
    Absolute,
    Relative,
}

impl Kind {
    pub fn is_relative(&self) -> bool {
        *self == Self::Relative
    }

    const fn text(&self) -> &'static str {
        match self {
            Self::Absolute => ABSOLUTE,
            Self::Relative => RELATIVE,
        }
    }
}
