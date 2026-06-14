use crate::r#const::{PERCENT, PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::{BOOKMARK, DOTS_THREE};
use egui_variant::EguiVariant;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision and significant
#[derive(Clone, Copy, Debug, Deserialize, EguiVariant, Hash, PartialEq, Serialize)]
#[egui_variant(name = DOTS_THREE)]
pub struct PrecisionAndSignificant {
    #[egui_variant(name = _ui.localize(formatcp!("{PREFIX}_{PRECISION}")), range = 1..=MAX_PRECISION, bookmarks = [1, 3])]
    pub precision: usize,
    #[egui_variant(name = _ui.localize(formatcp!("{PREFIX}_{SIGNIFICANT}")))]
    pub significant: bool,
}

impl Default for PrecisionAndSignificant {
    fn default() -> Self {
        Self {
            precision: 1,
            significant: false,
        }
    }
}
