pub mod l10n {
    use egui_l10n::asset;

    pub const EN: &[&str] = &[asset!("/ftl/en/attributes.ftl")];

    pub const RU: &[&str] = &[asset!("/ftl/ru/attributes.ftl")];
}

pub mod buttons;
pub mod r#const;
pub mod settings;

#[cfg(feature = "fatty_acids")]
pub mod fatty_acids;

#[cfg(feature = "polars")]
pub mod polars;

pub(crate) mod utils;
