#![feature(result_option_map_or_default)]

use egui::Ui;

pub mod l10n {
    use egui_l10n::ftl;

    pub const EN: &[&str] = &[ftl!("en/main.ftl")];

    pub const RU: &[&str] = &[ftl!("ru/main.ftl")];
}

pub mod buttons;
pub mod r#const;
pub mod settings;

#[cfg(feature = "polars")]
pub mod polars;
#[cfg(feature = "variants")]
pub mod variants;

pub(crate) mod utils;
