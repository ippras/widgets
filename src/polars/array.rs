use crate::r#const::{ARRAY, EM_DASH, MEAN, NO_BREAK_SPACE, STANDARD_DEVIATION};
use egui::{Color32, Response, TextWrapMode, Ui, WidgetText};
use egui_l20n::prelude::*;
use itertools::Itertools;
use polars::prelude::*;
use polars_ext::option::DisplayOption;
use polars_utils::format_list;
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

/// Float 64 array (mean and standard deviation)
#[derive(TypedBuilder)]
pub struct Float64Array<'a> {
    series: &'a Series,
    row: usize,
    mean: bool,
    standard_deviation: bool,
    #[builder(default, setter(strip_option))]
    color: Option<Color32>,
}

impl Float64Array<'_> {
    pub fn show(&self, ui: &mut Ui) -> PolarsResult<Response> {
        let array_series = self.series.struct_()?.field_by_name(ARRAY)?;
        let array = array_series.array()?.get_as_series(self.row);
        let mean_series = self.series.struct_()?.field_by_name(MEAN)?;
        let mean = mean_series.f64()?.get(self.row);
        let standard_deviation_series = self.series.struct_()?.field_by_name(STANDARD_DEVIATION)?;
        let standard_deviation = standard_deviation_series.f64()?.get(self.row);
        let mut text = if self.mean
            && self.standard_deviation
            && let Some(mean) = mean
            && let Some(standard_deviation) = standard_deviation
        {
            WidgetText::from(format!("{mean}{NO_BREAK_SPACE}±{standard_deviation}"))
        } else if self.mean {
            WidgetText::from(mean.display().to_string())
        } else if let Some(array) = array {
            WidgetText::from(format!(
                "[{}]",
                array
                    .f64()?
                    .iter()
                    .format_with(", ", |option, f| f(&option.display()))
            ))
        } else {
            WidgetText::from(EM_DASH)
        };
        if let Some(color) = self.color {
            text = text.color(color);
        }
        let mut response = ui.label(text);
        if response.hovered() {
            // Mean
            if let Some(mean) = mean {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(MEAN));
                    ui.label(mean.to_string());
                });
            }
            // Standard deviation
            if let Some(standard_deviation) = standard_deviation {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(STANDARD_DEVIATION));
                    ui.label(format!("±{standard_deviation}"));
                });
            }
            // Array
            if let Some(sample) = self
                .series
                .struct_()?
                .field_by_name(ARRAY)?
                .array()?
                .get_as_series(self.row)
                && sample.len() > 1
            {
                response = response.on_hover_ui(|ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    ui.heading(ui.localize(ARRAY));
                    ui.label(format_list!(sample.iter()));
                });
            }
        }
        Ok(response)
    }
}

/// Boolean array
#[derive(TypedBuilder)]
pub struct BooleanArray<T> {
    series: T,
    row: usize,
}

impl<T: Borrow<Series>> BooleanArray<T> {
    pub fn show(&self, ui: &mut Ui) -> PolarsResult<Response> {
        let r#struct = self.series.borrow().struct_()?;
        let arrays_series = r#struct.field_by_name(ARRAY)?;
        let text = arrays_series
            .array()?
            .get_as_series(self.row)
            .map(|series| -> PolarsResult<_> {
                Ok(format!(
                    "[{}]",
                    series
                        .bool()?
                        .iter()
                        .format_with(", ", |option, f| f(&option.display()))
                ))
            })
            .transpose()?
            .display()
            .to_string();

        // let text = match arrays_series.array()?.get_as_series(self.row) {
        //     None => WidgetText::from(EM_DASH),
        //     Some(array_series) => WidgetText::from(format!(
        //         "[{}]",
        //         array_series
        //             .bool()?
        //             .iter()
        //             .format_with(", ", |option, f| f(&option.display()))
        //     )),
        // };
        let mut response = ui.label(text);
        if response.hovered() {}
        //     // Mean
        //     if let Some(mean) = mean {
        //         response = response.on_hover_ui(|ui| {
        //             ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        //             ui.heading(ui.localize(MEAN));
        //             ui.label(mean.to_string());
        //         });
        //     }
        //     // Standard deviation
        //     if let Some(standard_deviation) = standard_deviation {
        //         response = response.on_hover_ui(|ui| {
        //             ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        //             ui.heading(ui.localize(STANDARD_DEVIATION));
        //             ui.label(format!("±{standard_deviation}"));
        //         });
        //     }
        //     // Array
        //     if let Some(sample) = self
        //         .series
        //         .struct_()?
        //         .field_by_name(ARRAY)?
        //         .array()?
        //         .get_as_series(self.row)
        //         && sample.len() > 1
        //     {
        //         response = response.on_hover_ui(|ui| {
        //             ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        //             ui.heading(ui.localize(ARRAY));
        //             ui.label(format_list!(sample.iter()));
        //         });
        //     }
        // }
        Ok(response)
    }
}
