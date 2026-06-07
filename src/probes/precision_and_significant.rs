use crate::r#const::{PERCENT, PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::{BOOKMARK, DOTS_THREE_OUTLINE};
use egui_probe::EguiProbe;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// IEEE 754-2008
pub const MAX_PRECISION: usize = 16;

/// Precision and significant
#[derive(Clone, Copy, Debug, Deserialize, EguiProbe, Hash, PartialEq, Serialize)]
#[egui_probe(name = DOTS_THREE_OUTLINE)]
pub struct PrecisionAndSignificant {
    #[egui_probe(name = _ui.localize(formatcp!("{PREFIX}_{PRECISION}")), range = 1..=MAX_PRECISION, bookmarks = [1, 3])]
    pub precision: usize,
    #[egui_probe(name = _ui.localize(formatcp!("{PREFIX}_{SIGNIFICANT}")))]
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
