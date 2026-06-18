use crate::{
    r#const::{
        ACTION, AUTO, EM_DASH, FILTER, HIGHLIGHT, KIND, MANUAL, OPERATOR, PREFIX, SORT,
        SORT_BY_MINOR_MAJOR, THRESHOLD,
    },
    settings::HighlightSortFilter,
    utils::format_list_truncated,
};
use const_format::formatcp;
use egui::{
    ComboBox, IntoAtoms, PopupCloseBehavior, Slider, SliderClamping, TextWrapMode, Ui, Widget,
};
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
    pub bookmarks: Vec<OrderedFloat<f64>>,
    #[builder(default)]
    pub action: Action,
}

impl Threshold {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn show(&mut self, ui: &mut Ui, lipids: &[String], percent: bool) {
        self.kind(ui);
        self.auto(ui, percent);
        self.manual(ui, lipids);

        ui.separator();

        self.action(ui);
    }

    /// Kind
    fn kind(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{KIND}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{KIND}.hover")));
                });
            for kind in [Kind::Auto, Kind::Manual] {
                ui.selectable_value(&mut self.kind, kind, ui.localize(kind.text()))
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(kind.hover_text()));
                    });
            }
        });
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
                ui.menu_button(BOOKMARK, |ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    for bookmark in &self.bookmarks {
                        let text = if percent {
                            format!("{}%", bookmark * 100.0)
                        } else {
                            bookmark.to_string()
                        };
                        if ui
                            .selectable_value(&mut self.auto, *bookmark, text)
                            .changed()
                        {
                            self.auto = *bookmark;
                            self.kind = Kind::Auto;
                        }
                    }
                });
            }
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
                    let mut index = 0;
                    for (lipid, selected) in zip(lipids, &mut self.manual) {
                        let index = if *selected {
                            index.to_string()
                        } else {
                            EM_DASH.to_string()
                        };
                        if ui
                            .toggle_value(selected, (index, lipid))
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

    /// Action
    fn action(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{ACTION}")))
                .on_hover_ui(|ui| {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{THRESHOLD}_{ACTION}.hover")));
                });
            for action in [Action::Highlight, Action::Sort, Action::Filter] {
                ui.selectable_value(&mut self.action, action, ui.localize(action.text()))
                    .on_hover_ui(|ui| {
                        ui.label(ui.localize(action.hover_text()));
                    });
            }
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

/// Threshold action
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Action {
    #[default]
    Highlight,
    Sort,
    Filter,
}

impl Action {
    pub const fn text(&self) -> &'static str {
        match self {
            Self::Highlight => formatcp!("{PREFIX}_{HIGHLIGHT}"),
            Self::Sort => formatcp!("{PREFIX}_{SORT}"),
            Self::Filter => formatcp!("{PREFIX}_{FILTER}"),
        }
    }

    pub const fn hover_text(&self) -> &'static str {
        match self {
            Self::Highlight => formatcp!("{PREFIX}_{HIGHLIGHT}.hover"),
            Self::Sort => formatcp!("{PREFIX}_{SORT}.hover"),
            Self::Filter => formatcp!("{PREFIX}_{FILTER}.hover"),
        }
    }
}

/// Threshold kind
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

impl Kind {
    pub fn is_auto(&self) -> bool {
        *self == Self::Auto
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
