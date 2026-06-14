use crate::r#const::{
    ABSOLUTE, DELTA_DEGREES_OF_FREEDOM, KIND, MEAN, PREFIX, RELATIVE, RELATIVE_STANDARD_DEVIATION,
    STANDARD_DEVIATION,
};
use const_format::formatcp;
use display_option::DisplayOption;
use egui::{ComboBox, Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::DOTS_THREE;
use egui_variant::EguiVariant;
use serde::{Deserialize, Serialize};

/// Mean and standard deviation
#[derive(Clone, Copy, Debug, Default, Deserialize, EguiVariant, Hash, PartialEq, Serialize)]
#[egui_variant(name = DOTS_THREE)]
pub struct MeanAndStandardDeviation {
    #[egui_variant(name = _ui.localize(formatcp!("{PREFIX}_{MEAN}")))]
    pub mean: Option<StandardDeviation>,
}

/// Mean
#[derive(Clone, Copy, Debug, Default, Deserialize, EguiVariant, Hash, PartialEq, Serialize)]
#[egui_variant(name = DOTS_THREE)]
pub struct StandardDeviation {
    #[egui_variant(name = _ui.localize(formatcp!("{PREFIX}_{STANDARD_DEVIATION}")))]
    pub standard_deviation: Option<KindAndDdof>,
}

/// Standard deviation
#[derive(Clone, Copy, Debug, Deserialize, EguiVariant, Hash, PartialEq, Serialize)]
#[egui_variant(name = DOTS_THREE)]
pub struct KindAndDdof {
    #[egui_variant(name = _ui.localize(formatcp!("{PREFIX}_{KIND}")))]
    pub kind: Kind,
    #[egui_variant(name = _ui.localize(formatcp!("{PREFIX}_{DELTA_DEGREES_OF_FREEDOM}")), range = 0..=1, bookmarks = [1])]
    pub ddof: u8,
}

impl Default for KindAndDdof {
    fn default() -> Self {
        Self {
            kind: Kind::Absolute,
            ddof: 1,
        }
    }
}

/// Standard deviation kind
#[derive(Clone, Copy, Debug, Default, Deserialize, EguiVariant, Hash, PartialEq, Serialize)]
#[egui_variant(combobox)]
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
