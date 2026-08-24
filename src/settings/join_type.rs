use crate::r#const::{DIFFERENCE, INTERSECTION, JOIN_TYPE, PREFIX, UNION};
use const_format::formatcp;
use egui::{ComboBox, RichText, Ui};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::{EXCLUDE, INTERSECT, UNITE};

/// Join type
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JoinType {
    Difference,
    Intersection,
    #[default]
    Union,
}

impl JoinType {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Difference => EXCLUDE,
            Self::Intersection => INTERSECT,
            Self::Union => UNITE,
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            Self::Difference => formatcp!("{PREFIX}_{JOIN_TYPE}_{DIFFERENCE}"),
            Self::Intersection => formatcp!("{PREFIX}_{JOIN_TYPE}_{INTERSECTION}"),
            Self::Union => formatcp!("{PREFIX}_{JOIN_TYPE}_{UNION}"),
        }
    }

    pub fn hover_text(&self) -> &'static str {
        match self {
            Self::Difference => formatcp!("{PREFIX}_{JOIN_TYPE}_{DIFFERENCE}.hover"),
            Self::Intersection => formatcp!("{PREFIX}_{JOIN_TYPE}_{INTERSECTION}.hover"),
            Self::Union => formatcp!("{PREFIX}_{JOIN_TYPE}_{UNION}.hover"),
        }
    }
}

impl JoinType {
    pub fn show(&mut self, ui: &mut Ui) {
        const ID: &str = formatcp!("{PREFIX}_{JOIN_TYPE}");

        ui.horizontal(|ui| {
            ui.label(ui.localize(ID)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{ID}.hover")));
            });
            ComboBox::from_id_salt(ui.make_persistent_id(ID).with("ComboBox"))
                .selected_text(ui.localize(self.text()))
                .show_ui(ui, |ui| {
                    for join_type in [Self::Intersection, Self::Union, Self::Difference] {
                        ui.selectable_value(self, join_type, ui.localize(join_type.text()))
                            .on_hover_ui(|ui| {
                                ui.label(ui.localize(join_type.hover_text()));
                            });
                    }
                })
                .response
                .on_hover_text(RichText::new(self.icon()).heading());
        });
    }
}
