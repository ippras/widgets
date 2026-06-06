use crate::r#const::{
    ABSOLUTE, DELTA_DEGREES_OF_FREEDOM, KIND, MEAN, PREFIX, RELATIVE, RELATIVE_STANDARD_DEVIATION,
    STANDARD_DEVIATION,
};
use const_format::formatcp;
use display_option::DisplayOption;
use egui::{ComboBox, Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_probe::EguiProbe;
use serde::{Deserialize, Serialize};

/// Mean and standard deviation
#[derive(Clone, Copy, Debug, Default, Deserialize, EguiProbe, Hash, PartialEq, Serialize)]
pub struct _MeanAndStandardDeviation {
    #[egui_probe(name = _ui.localize(formatcp!("{PREFIX}_{MEAN}")))]
    pub mean: Option<Mean>,
}

/// Mean
#[derive(Clone, Copy, Debug, Default, Deserialize, EguiProbe, Hash, PartialEq, Serialize)]
#[egui_probe(name = ui.localize(formatcp!("{PREFIX}_{MEAN}")))]
pub struct Mean {
    #[egui_probe(name = _ui.localize(formatcp!("{PREFIX}_{STANDARD_DEVIATION}")))]
    pub standard_deviation: Option<StandardDeviation>,
}

/// Standard deviation
#[derive(Clone, Copy, Debug, Deserialize, EguiProbe, Hash, PartialEq, Serialize)]
#[egui_probe(name = ui.localize(formatcp!("{PREFIX}_{STANDARD_DEVIATION}")))]
pub struct StandardDeviation {
    #[egui_probe(name = _ui.localize(formatcp!("{PREFIX}_{KIND}")))]
    pub kind: Kind,
    #[egui_probe(name = _ui.localize(formatcp!("{PREFIX}_{DELTA_DEGREES_OF_FREEDOM}")), range = 0..=1)]
    pub ddof: u8,
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self {
            kind: Kind::Absolute,
            ddof: 1,
        }
    }
}

/// Mean and standard deviation
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct MeanAndStandardDeviation {
    pub mean: bool,
    pub standard_deviation: bool,
    pub kind: Kind,
    pub ddof: u8,
}

impl MeanAndStandardDeviation {
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
            ui.label(ui.localize(formatcp!("{PREFIX}_{MEAN}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{MEAN}.hover")));
                });
            ui.checkbox(&mut self.mean, ());
        });

        ui.horizontal(|ui| {
            if !self.mean {
                ui.disable();
            }
            ui.label(ui.localize(formatcp!("{PREFIX}_{STANDARD_DEVIATION}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{STANDARD_DEVIATION}.hover")));
                });
            ui.checkbox(&mut self.standard_deviation, ());
            if !self.standard_deviation {
                ui.disable();
            }
            ComboBox::from_id_salt(ui.next_auto_id())
                .selected_text(ui.localize(self.kind.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.kind,
                        Kind::Absolute,
                        ui.localize(Kind::Absolute.text()),
                    );
                    ui.selectable_value(
                        &mut self.kind,
                        Kind::Relative,
                        ui.localize(Kind::Relative.text()),
                    );
                });
        });

        // [DDOF](https://numpy.org/devdocs/reference/generated/numpy.std.html)
        ui.horizontal(|ui| {
            if !self.mean || !self.standard_deviation {
                ui.disable();
            }
            ui.label(ui.localize(formatcp!("{PREFIX}_{DELTA_DEGREES_OF_FREEDOM}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!(
                        "{PREFIX}_{DELTA_DEGREES_OF_FREEDOM}.abbreviation"
                    )));
                })
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{DELTA_DEGREES_OF_FREEDOM}.hover")));
                });
            Slider::new(&mut self.ddof, 0..=1)
                .update_while_editing(false)
                .ui(ui);
        });
    }
}

/// Standard deviation kind
#[derive(Clone, Copy, Debug, Default, Deserialize, EguiProbe, Hash, PartialEq, Serialize)]
#[egui_probe(tags combobox)]
pub enum Kind {
    #[default]
    Absolute,
    Relative,
}

impl Kind {
    pub fn is_relative(&self) -> bool {
        *self == Self::Relative
    }

    const fn text(self) -> &'static str {
        match self {
            Self::Absolute => formatcp!("{PREFIX}_{ABSOLUTE}"),
            Self::Relative => formatcp!("{PREFIX}_{RELATIVE_STANDARD_DEVIATION}.short"),
        }
    }
}
