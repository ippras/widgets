use crate::{
    r#const::{
        AUTO_THRESHOLD, FILTER_THRESHOLD, IS_AUTO_THRESHOLD, MANUAL_THRESHOLD, OPERATOR,
        SORT_BY_MINOR_MAJOR,
    },
    l10n,
    utils::format_list_truncated,
};
use egui::{ComboBox, PopupCloseBehavior, Slider, SliderClamping, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::BOOKMARK;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::iter::zip;
use typed_builder::TypedBuilder;

/// Threshold
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, TypedBuilder)]
pub struct Threshold {
    #[builder(default, setter(skip))]
    pub auto: OrderedFloat<f64>,
    #[builder(default, setter(skip))]
    pub filter: bool,
    #[builder(default = true, setter(skip))]
    pub is_auto: bool,
    #[builder(default, setter(skip))]
    pub manual: Vec<bool>,
    #[builder(default = Operator::Max, setter(skip))]
    pub operator: Operator,
    #[builder(default, setter(skip))]
    pub sort: bool,

    #[builder(default, setter(into, strip_option))]
    bookmark: Option<OrderedFloat<f64>>,
}

impl Threshold {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn show(&mut self, ui: &mut Ui, lipids: &[String], percent: bool) {
        self.auto(ui, percent);
        self.manual(ui, lipids);
        self.sort(ui);
        self.filter(ui);

        self.operator(ui);
    }

    /// Auto threshold
    fn auto(&mut self, ui: &mut Ui, percent: bool) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(AUTO_THRESHOLD)))
                .on_hover_localized(l10n!(AUTO_THRESHOLD; hover));
            ui.checkbox(&mut self.is_auto, ())
                .on_hover_localized(l10n!(IS_AUTO_THRESHOLD; hover));
            if !self.is_auto {
                ui.disable();
            }
            if Slider::new(&mut self.auto.0, 0.0..=1.0)
                .clamping(SliderClamping::Always)
                .custom_formatter(|mut value, _| {
                    if percent {
                        value *= 100.0;
                    }
                    AnyValue::Float64(value).to_string()
                })
                .custom_parser(|value| {
                    let mut parsed = value.parse().ok()?;
                    if percent {
                        parsed /= 100.0;
                    }
                    Some(parsed)
                })
                .logarithmic(true)
                .update_while_editing(false)
                .ui(ui)
                .changed()
            {
                self.is_auto = true;
            }
            if let Some(bookmark) = self.bookmark {
                let text = if percent {
                    format!("{}%", bookmark * 100.0)
                } else {
                    bookmark.to_string()
                };
                if ui.button((BOOKMARK, text)).clicked() {
                    self.auto = bookmark;
                    self.is_auto = true;
                }
            }
        });
    }

    /// Manual threshold
    fn manual(&mut self, ui: &mut Ui, lipids: &[String]) {
        ui.horizontal(|ui| {
            if self.is_auto {
                ui.disable();
            }
            ui.label(ui.localize(l10n!(MANUAL_THRESHOLD)))
                .on_hover_localized(l10n!(MANUAL_THRESHOLD; hover));
            let selected_text = format_list_truncated(
                zip(&self.manual, lipids).filter_map(|(keep, lipid)| keep.then_some(lipid)),
            );
            ComboBox::from_id_salt("ManualThreshold")
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .selected_text(&selected_text)
                .show_ui(ui, |ui| {
                    for (lipid, selected) in zip(lipids, &mut self.manual) {
                        if ui
                            .toggle_value(selected, lipid)
                            .on_hover_text(lipid)
                            .changed()
                        {
                            self.is_auto = false;
                        }
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(selected_text);
                });
        });
    }

    /// Filter thresholded
    fn filter(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(FILTER_THRESHOLD)))
                .on_hover_localized(l10n!(FILTER_THRESHOLD; hover));
            ui.checkbox(&mut self.filter, ());
        });
    }

    /// Sort thresholded
    fn sort(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Sort by minor major
            ui.label(ui.localize(l10n!(SORT_BY_MINOR_MAJOR)))
                .on_hover_localized(l10n!(SORT_BY_MINOR_MAJOR; hover));
            ui.checkbox(&mut self.sort, ());
        });
    }

    /// Operator
    fn operator(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(l10n!(OPERATOR)))
                .on_hover_localized(l10n!(OPERATOR; hover));
            ComboBox::from_id_salt("Operator")
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .selected_text(&ui.localize(self.operator.text()))
                .show_ui(ui, |ui| {
                    for selected_value in [
                        Operator::Max,
                        Operator::Min,
                        Operator::Mean,
                        Operator::Median,
                    ] {
                        ui.selectable_value(
                            &mut self.operator,
                            selected_value,
                            ui.localize(selected_value.text()),
                        )
                        .on_hover_localized(self.operator.hover_text());
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(self.operator.hover_text()));
                });
        });
    }
}

/// Operator
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub enum Operator {
    #[default]
    Max,
    Min,
    Mean,
    Median,
}

impl Operator {
    pub const fn text(&self) -> &'static str {
        match self {
            Self::Max => "Max",
            Self::Min => "Min",
            Self::Mean => "Mean",
            Self::Median => "Median",
        }
    }

    pub const fn hover_text(&self) -> &'static str {
        match self {
            Self::Max => "Max.hover",
            Self::Min => "Min.hover",
            Self::Mean => "Mean.hover",
            Self::Median => "Median.hover",
        }
    }
}
