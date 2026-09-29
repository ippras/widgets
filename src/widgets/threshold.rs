use crate::{
    r#const::{ACTION, AUTO, KIND, MANUAL, PREFIX, THRESHOLD},
    settings::HighlightSortFilter,
    utils::format_list_truncated,
};
use const_format::formatcp;
use egui::{ComboBox, Grid, PopupCloseBehavior, Slider, SliderClamping, TextWrapMode, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use std::iter::zip;
use typed_builder::TypedBuilder;

/// Auto threshold
#[derive(Debug, TypedBuilder)]
pub struct AutoThreshold<'a> {
    pub value: &'a mut f64,
    #[builder(default)]
    pub bookmarks: &'a [f64],
    #[builder(default)]
    pub percent: bool,
}

impl AutoThreshold<'_> {
    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::Response {
        const ID: &str = formatcp!("{PREFIX}_{THRESHOLD}");

        ui.horizontal(|ui| {
            ui.label(ui.localize(ID)).on_hover_ui(|ui| {
                ui.label(ui.localize(formatcp!("{ID}.hover")));
            });
            let mut response = egui::Slider::new(self.value, 0.0..=1.0)
                .clamping(egui::SliderClamping::Always)
                .custom_formatter(|mut value, _| {
                    if self.percent {
                        value *= 100.0;
                    }
                    AnyValue::Float64(value).to_string()
                })
                .custom_parser(|value| {
                    let mut parsed = value.parse().ok()?;
                    if self.percent {
                        parsed /= 100.0;
                    }
                    Some(parsed)
                })
                .logarithmic(true)
                .update_while_editing(false)
                .ui(ui);
            if !self.bookmarks.is_empty() {
                ui.menu_button(BOOKMARK, |ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    for bookmark in self.bookmarks {
                        let text = if self.percent {
                            format!("{}%", bookmark * 100.0)
                        } else {
                            bookmark.to_string()
                        };
                        if ui.selectable_value(self.value, *bookmark, text).changed() {
                            *self.value = *bookmark;
                            response.mark_changed();
                        }
                    }
                });
            }
            response
        })
        .inner
    }
}
