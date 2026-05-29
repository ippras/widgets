use crate::r#const::{BY_KEY, BY_VALUE, ORDER, PREFIX, SORT};
use const_format::formatcp;
use egui::{ComboBox, Ui};
use egui_l10n::ContextExt as _;
use serde::{Deserialize, Serialize};

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Sort {
    pub checked: bool,
    pub kind: SortKind,
}

impl Sort {
    pub fn new() -> Self {
        Self {
            checked: false,
            kind: SortKind::Value,
        }
    }
}

impl Sort {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SORT}")));
            ui.checkbox(&mut self.checked, ());
            if !self.checked {
                ui.disable();
            }
            ComboBox::from_id_salt(SORT)
                .selected_text(ui.localize(self.kind.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.kind,
                        SortKind::Key,
                        ui.localize(SortKind::Key.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(SortKind::Key.hover_text()));
                    });
                    ui.selectable_value(
                        &mut self.kind,
                        SortKind::Value,
                        ui.localize(SortKind::Value.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(SortKind::Value.hover_text()));
                    });
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.kind.hover_text()));
                });
        });
    }
}

/// By
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SortKind {
    Key,
    Value,
}

impl SortKind {
    const fn text(&self) -> &'static str {
        match self {
            SortKind::Key => formatcp!("{PREFIX}_{BY_KEY}"),
            SortKind::Value => formatcp!("{PREFIX}_{BY_VALUE}"),
        }
    }

    const fn hover_text(&self) -> &'static str {
        match self {
            SortKind::Key => formatcp!("{PREFIX}_{BY_KEY}.hover"),
            SortKind::Value => formatcp!("{PREFIX}_{BY_VALUE}.hover"),
        }
    }
}
