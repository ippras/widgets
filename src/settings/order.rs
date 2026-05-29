use crate::r#const::{ASCENDING, BY_KEY, BY_VALUE, DESCENDING, ORDER, PREFIX, SORT};
use const_format::formatcp;
use egui::{ComboBox, Ui};
use egui_l10n::ContextExt as _;
use serde::{Deserialize, Serialize};

/// Order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Order {
    pub kind: OrderKind,
}

impl Order {
    pub fn new() -> Self {
        Self {
            kind: OrderKind::Ascending,
        }
    }
}

impl Order {
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{ORDER}")));
            ComboBox::from_id_salt(ORDER)
                .selected_text(ui.localize(self.kind.text()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.kind,
                        OrderKind::Ascending,
                        ui.localize(OrderKind::Ascending.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(OrderKind::Ascending.hover_text()));
                    });
                    ui.selectable_value(
                        &mut self.kind,
                        OrderKind::Descending,
                        ui.localize(OrderKind::Descending.text()),
                    )
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(OrderKind::Descending.hover_text()));
                    });
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.kind.hover_text()));
                });
        });
    }
}

/// Order kind
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum OrderKind {
    #[default]
    Ascending,
    Descending,
}

impl OrderKind {
    pub fn is_descending(&self) -> bool {
        match self {
            OrderKind::Ascending => false,
            OrderKind::Descending => true,
        }
    }
}
impl OrderKind {
    pub const fn text(&self) -> &'static str {
        match self {
            OrderKind::Ascending => formatcp!("{PREFIX}_{ASCENDING}"),
            OrderKind::Descending => formatcp!("{PREFIX}_{DESCENDING}"),
        }
    }

    pub const fn hover_text(&self) -> &'static str {
        match self {
            OrderKind::Ascending => formatcp!("{PREFIX}_{ASCENDING}.hover"),
            OrderKind::Descending => formatcp!("{PREFIX}_{DESCENDING}.hover"),
        }
    }
}
