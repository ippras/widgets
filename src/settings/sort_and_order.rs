use crate::settings::{Sort, order::Order};
use egui::Ui;
use serde::{Deserialize, Serialize};

/// Sort and order
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct SortAndOrder {
    pub sort: Sort,
    pub order: Order,
}

impl SortAndOrder {
    pub fn new() -> Self {
        Self {
            sort: Sort::new(),
            order: Order::new(),
        }
    }
}

impl SortAndOrder {
    pub fn show(&mut self, ui: &mut Ui) {
        self.sort.show(ui);
        self.order.show(ui);
    }
}
