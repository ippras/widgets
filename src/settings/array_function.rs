use crate::r#const::{ARRAY_FUNCTION, PREFIX};
use const_format::formatcp;
use egui::{ComboBox, Ui};
use egui_l10n::ContextExt as _;

/// Array function
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArrayFunction {
    #[default]
    Max,
    Min,
    Mean,
    Median,
}

impl ArrayFunction {
    pub fn show(&mut self, ui: &mut Ui) {
        const ID: &str = formatcp!("{PREFIX}_{ARRAY_FUNCTION}");

        ui.horizontal(|ui| {
            ui.label(ui.localize(ID)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{ID}.hover")));
            });
            ComboBox::from_id_salt(ui.make_persistent_id(ID).with("ComboBox"))
                .selected_text(ui.localize(self.text()))
                .show_ui(ui, |ui| {
                    for array_function in [Self::Max, Self::Min, Self::Mean, Self::Median] {
                        ui.selectable_value(
                            self,
                            array_function,
                            ui.localize(array_function.text()),
                        )
                        .on_hover_ui(|ui| {
                            ui.label(ui.localize(array_function.hover_text()));
                        });
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.hover_text()));
                });
        });
    }
}

impl ArrayFunction {
    pub const fn text(&self) -> &'static str {
        match self {
            Self::Max => "Max",
            Self::Min => "Min",
            Self::Mean => "Mean",
            Self::Median => "Median",
        }
    }

    pub const fn hover_text(&self) -> &'static str {
        match self {
            Self::Max => "Max.hover",
            Self::Min => "Min.hover",
            Self::Mean => "Mean.hover",
            Self::Median => "Median.hover",
        }
    }
}
