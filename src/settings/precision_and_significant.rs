use crate::r#const::{PRECISION, PREFIX, SIGNIFICANT};
use const_format::formatcp;
use egui::{Slider, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use typed_builder::TypedBuilder;

/// Precision and significant settings
#[derive(Clone, Copy, Debug, Hash, PartialEq, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrecisionAndSignificant {
    #[builder(default = 1)]
    pub precision: usize,
    #[builder(default)]
    pub significant: bool,
}

impl PrecisionAndSignificant {
    pub fn new() -> Self {
        Self::builder().build()
    }
}
