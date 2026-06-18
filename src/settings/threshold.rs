use crate::{
    r#const::{AUTO, KIND, MANUAL, OPERATOR, PREFIX, SORT_BY_MINOR_MAJOR, THRESHOLD},
    settings::HighlightSortFilter,
    utils::format_list_truncated,
};
use const_format::formatcp;
use egui::{ComboBox, PopupCloseBehavior, Slider, SliderClamping, Ui, Widget};
use egui_l10n::ContextExt as _;
use egui_phosphor::regular::BOOKMARK;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use std::iter::zip;
use typed_builder::TypedBuilder;

/// Threshold
#[derive(Clone, Debug, Hash, PartialEq, TypedBuilder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Threshold {
    #[builder(default, setter(skip))]
    pub kind: Kind,
    #[builder(default, setter(skip))]
    pub auto: OrderedFloat<f64>,
    #[builder(default, setter(skip))]
    pub manual: Vec<bool>,
    // #[builder(default = Operator::Max, setter(skip))]
    // pub operator: Operator,
    // #[builder(default, setter(into, strip_option))]
    // bookmark: Option<OrderedFloat<f64>>,
    #[builder(default, via_mutators, mutators(
        pub fn bookmark(self, value: f64) {
            self.bookmarks.push(OrderedFloat(value));
        }
    ))]
    bookmarks: Vec<OrderedFloat<f64>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let t = Threshold::builder().bookmark(0.5).bookmark(1.0).build();
    }
}

impl Threshold {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn show(&mut self, ui: &mut Ui, lipids: &[String], percent: bool) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{KIND}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{KIND}.hover")));
                });
            ui.selectable_value(
                &mut self.kind,
                Kind::Auto,
                ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{AUTO}")),
            );
            ui.selectable_value(
                &mut self.kind,
                Kind::Manual,
                ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{MANUAL}")),
            );
        });
        self.auto(ui, percent);
        self.manual(ui, lipids);
    }

    /// Auto threshold
    fn auto(&mut self, ui: &mut Ui, percent: bool) {
        ui.horizontal(|ui| {
            if self.kind != Kind::Auto {
                ui.disable();
            }
            ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{AUTO}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{AUTO}.hover")));
                });
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
                self.kind = Kind::Auto;
            }
            if !self.bookmarks.is_empty() {
                for bookmark in &self.bookmarks {
                    let text = if percent {
                        format!("{}%", bookmark * 100.0)
                    } else {
                        bookmark.to_string()
                    };
                    if ui.button((BOOKMARK, text)).clicked() {
                        self.auto = *bookmark;
                        self.kind = Kind::Auto;
                    }
                }
            }
            // if let Some(bookmark) = self.bookmark {
            //     let text = if percent {
            //         format!("{}%", bookmark * 100.0)
            //     } else {
            //         bookmark.to_string()
            //     };
            //     if ui.button((BOOKMARK, text)).clicked() {
            //         self.auto = bookmark;
            //         self.kind = Kind::Auto;
            //     }
            // }
        });
    }

    /// Manual threshold
    fn manual(&mut self, ui: &mut Ui, lipids: &[String]) {
        ui.horizontal(|ui| {
            if self.kind != Kind::Manual {
                ui.disable();
            }
            ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{MANUAL}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{MANUAL}.hover")));
                });
            let selected_text = format_list_truncated(
                zip(&self.manual, lipids).filter_map(|(keep, lipid)| keep.then_some(lipid)),
            );
            ComboBox::from_id_salt(ui.next_auto_id())
                .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                .selected_text(&selected_text)
                .show_ui(ui, |ui| {
                    for (lipid, selected) in zip(lipids, &mut self.manual) {
                        if ui
                            .toggle_value(selected, lipid)
                            .on_hover_text(lipid)
                            .changed()
                        {
                            self.kind = Kind::Manual;
                        }
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    ui.label(selected_text);
                });
        });
    }

    // /// Operator
    // fn operator(&mut self, ui: &mut Ui) {
    //     ui.horizontal(|ui| {
    //         ui.label(ui.localize(formatcp!("{PREFIX}_{OPERATOR}")))
    //             .on_hover_ui(|ui| {
    //                 ui.label(ui.localize(formatcp!("{PREFIX}_{OPERATOR}.hover")));
    //             });
    //         ComboBox::from_id_salt("Operator")
    //             .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
    //             .selected_text(&ui.localize(self.operator.text()))
    //             .show_ui(ui, |ui| {
    //                 for selected_value in [
    //                     Operator::Max,
    //                     Operator::Min,
    //                     Operator::Mean,
    //                     Operator::Median,
    //                 ] {
    //                     ui.selectable_value(
    //                         &mut self.operator,
    //                         selected_value,
    //                         ui.localize(selected_value.text()),
    //                     )
    //                     .on_hover_ui(|ui| {
    //                         ui.label(ui.localize(self.operator.hover_text()));
    //                     });
    //                 }
    //             })
    //             .response
    //             .on_hover_ui(|ui| {
    //                 ui.label(ui.localize(self.operator.hover_text()));
    //             });
    //     });
    // }
}

/// Standard deviation kind
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    #[default]
    Auto,
    Manual,
}

impl Kind {
    pub const fn text(&self) -> &'static str {
        match self {
            Self::Auto => formatcp!("{PREFIX}_{THRESHOLD}_{AUTO}"),
            Self::Manual => formatcp!("{PREFIX}_{THRESHOLD}_{MANUAL}"),
        }
    }

    pub const fn hover_text(&self) -> &'static str {
        match self {
            Self::Auto => formatcp!("{PREFIX}_{THRESHOLD}_{AUTO}.hover"),
            Self::Manual => formatcp!("{PREFIX}_{THRESHOLD}_{MANUAL}.hover"),
        }
    }
}

/// Operator
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
