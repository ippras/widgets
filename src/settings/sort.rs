use crate::r#const::{ASCENDING, BY_KEY, BY_VALUE, DESCENDING, ORDER, PREFIX, SORT};
use const_format::formatcp;
use egui::{ComboBox, Ui};
use egui_l10n::ContextExt as _;
use serde::{Deserialize, Serialize};

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Sort {
    pub checked: bool,
    pub by: By,
    pub order: Order,
}

impl Sort {
    pub fn new() -> Self {
        Self {
            checked: false,
            by: By::Value,
            order: Order::Descending,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        // Sort
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SORT}")));
            ui.checkbox(&mut self.checked, ());
            if !self.checked {
                ui.disable();
            }
            ComboBox::from_id_salt(SORT)
                .selected_text(ui.localize(self.by.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.by, By::Key, ui.localize(By::Key.text()))
                        .on_hover_ui(|ui| {
                            ui.label(ui.localize(By::Key.hover_text()));
                        });
                    ui.selectable_value(&mut self.by, By::Value, ui.localize(By::Value.text()))
                        .on_hover_ui(|ui| {
                            ui.label(ui.localize(By::Value.hover_text()));
                        });
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.by.hover_text()));
                });
        });

        // Order
        ui.horizontal(|ui| {
            if !self.checked {
                ui.disable();
            }
            ui.label(ui.localize(formatcp!("{PREFIX}_{ORDER}")));
            ComboBox::from_id_salt(ORDER)
                .selected_text(ui.localize(self.order.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.order,
                        Order::Ascending,
                        ui.localize(Order::Ascending.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(Order::Ascending.hover_text()));
                    });
                    ui.selectable_value(
                        &mut self.order,
                        Order::Descending,
                        ui.localize(Order::Descending.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(Order::Descending.hover_text()));
                    });
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.order.hover_text()));
                });
        });
    }
}

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn is_descending(&self) -> bool {
        match self {
            Order::Ascending => false,
            Order::Descending => true,
        }
    }
}
impl Order {
    const fn text(&self) -> &'static str {
        match self {
            Order::Ascending => formatcp!("{PREFIX}_{ASCENDING}"),
            Order::Descending => formatcp!("{PREFIX}_{DESCENDING}"),
        }
    }

    const fn hover_text(&self) -> &'static str {
        match self {
            Order::Ascending => formatcp!("{PREFIX}_{ASCENDING}.hover"),
            Order::Descending => formatcp!("{PREFIX}_{DESCENDING}.hover"),
        }
    }
}

/// By
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum By {
    Key,
    Value,
}

impl By {
    const fn text(&self) -> &'static str {
        match self {
            By::Key => formatcp!("{PREFIX}_{BY_KEY}"),
            By::Value => formatcp!("{PREFIX}_{BY_VALUE}"),
        }
    }

    const fn hover_text(&self) -> &'static str {
        match self {
            By::Key => formatcp!("{PREFIX}_{BY_KEY}.hover"),
            By::Value => formatcp!("{PREFIX}_{BY_VALUE}.hover"),
        }
    }
}
