use egui::Ui;

pub trait Show {
    fn show(&mut self, ui: &mut Ui);
}

pub mod l10n {
    use egui_l10n::ftl;

    pub const EN: &[&str] = &[ftl!("en/main.ftl")];

    pub const RU: &[&str] = &[ftl!("ru/main.ftl")];
}

pub mod buttons;
pub mod r#const;
pub mod settings;

#[cfg(feature = "fatty_acids")]
pub mod fatty_acids;

#[cfg(feature = "polars")]
pub mod polars;

pub(crate) mod utils;
