use crate::r#const::{ASCENDING, BY_KEY, BY_VALUE, DESCENDING, SORT};
use const_format::formatcp;
use egui::{ComboBox, Ui};
use egui_l10n::ContextExt as _;
use fatty_acid_expressions::r#const::PREFIX;
use serde::{Deserialize, Serialize};

/// Sort and order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Sort {
    pub order: Order,
    pub sort: Sort,
}

impl Sort {
    pub fn new() -> Self {
        Self {
            order: Order::Descending,
            sort: Sort::Value,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        self.sort(ui);
        self.order(ui);
    }

    /// Sort
    fn sort(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{SORT}")));
            ComboBox::from_id_salt(SORT)
                .selected_text(ui.localize(self.sort.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sort, Sort::Key, ui.localize(Sort::Key.text()))
                        .on_hover_ui(|ui| {
                            ui.label(ui.localize(Sort::Key.hover_text()));
                        });
                    ui.selectable_value(
                        &mut self.sort,
                        Sort::Value,
                        ui.localize(Sort::Value.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(Sort::Value.hover_text()));
                    });
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.sort.hover_text()));
                });
        });
    }

    /// Order
    fn order(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize("Order"));
            ComboBox::from_id_salt("Order")
                .selected_text(ui.localize(self.order.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.order, Order::Ascending, Order::Ascending.text())
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
    const fn text(&self) -> &'static str {
        match self {
            Order::Ascending => formatcp!("{PREFIX}_{ASCENDING}_{BY_KEY}"),
            Order::Descending => formatcp!("{PREFIX}_{DESCENDING}_{BY_KEY}"),
        }
    }

    const fn hover_text(&self) -> &'static str {
        match self {
            Order::Ascending => formatcp!("{PREFIX}_{ASCENDING}_{BY_KEY}.hover"),
            Order::Descending => formatcp!("{PREFIX}_{DESCENDING}_{BY_KEY}.hover"),
        }
    }
}

/// Sort
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Sort {
    Key,
    Value,
}

impl Sort {
    const fn text(&self) -> &'static str {
        match self {
            Sort::Key => formatcp!("{PREFIX}_{BY_KEY}"),
            Sort::Value => formatcp!("{PREFIX}_{BY_VALUE}"),
        }
    }

    const fn hover_text(&self) -> &'static str {
        match self {
            Sort::Key => formatcp!("{PREFIX}_{BY_KEY}.hover"),
            Sort::Value => formatcp!("{PREFIX}_{BY_VALUE}.hover"),
        }
    }
}
