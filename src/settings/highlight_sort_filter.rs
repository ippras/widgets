use crate::r#const::{ACTION, FILTER, HIGHLIGHT, PREFIX, SORT};
use const_format::formatcp;
use egui::Ui;
use egui_l10n::ContextExt as _;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

// /// Highlight, sort and filter variant
// #[derive(Clone, Debug, Default, Hash, PartialEq, TypedBuilder)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct HighlightSortFilterVariant {
//     pub value: HighlightSortFilter,
//     #[builder(default = formatcp!("{PREFIX}_{ACTION}").into())]
//     text: Cow<'static, str>,
//     #[builder(default = formatcp!("{PREFIX}_{ACTION}.hover").into())]
//     hover_text: Cow<'static, str>,
// }

// impl HighlightSortFilterVariant {
//     pub fn show(&mut self, ui: &mut Ui) {
//         ui.horizontal(|ui| {
//             ui.label(ui.localize(&self.text)).on_hover_ui(|ui| {
//                 ui.label(ui.localize(&self.hover_text));
//             });
//             for action in [
//                 HighlightSortFilter::Highlight,
//                 HighlightSortFilter::Sort,
//                 HighlightSortFilter::Filter,
//             ] {
//                 ui.selectable_value(&mut self.value, action, ui.localize(action.text()))
//                     .on_hover_ui(|ui| {
//                         ui.label(ui.localize(action.hover_text()));
//                     });
//             }
//         });
//     }
// }

/// Highlight, sort and filter
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HighlightSortFilter {
    #[default]
    Highlight,
    Sort,
    Filter,
}

impl HighlightSortFilter {
    pub const fn text(&self) -> &'static str {
        match self {
            Self::Highlight => formatcp!("{PREFIX}_{HIGHLIGHT}{SORT}{FILTER}_{HIGHLIGHT}"),
            Self::Sort => formatcp!("{PREFIX}_{HIGHLIGHT}{SORT}{FILTER}_{SORT}"),
            Self::Filter => formatcp!("{PREFIX}_{HIGHLIGHT}{SORT}{FILTER}_{FILTER}"),
        }
    }

    pub const fn hover_text(&self) -> &'static str {
        match self {
            Self::Highlight => formatcp!("{PREFIX}_{HIGHLIGHT}{SORT}{FILTER}_{HIGHLIGHT}.hover"),
            Self::Sort => formatcp!("{PREFIX}_{HIGHLIGHT}{SORT}{FILTER}_{SORT}.hover"),
            Self::Filter => formatcp!("{PREFIX}_{HIGHLIGHT}{SORT}{FILTER}_{FILTER}.hover"),
        }
    }
}
