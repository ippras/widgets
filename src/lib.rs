pub mod prelude {
    // pub use crate::{data_frame::DataFrameExt, expr::ExprExt, option::DisplayOption};

    // #[cfg(feature = "array")]
    // pub use crate::expr::array::{eval_arr, Array};

    // #[cfg(feature = "temporal_conversions")]
    // pub use polars_arrow::temporal_conversions::{
    //     timestamp_ms_to_datetime, timestamp_ns_to_datetime, timestamp_us_to_datetime,
    // };

    // pub mod r#const {
    //     pub use crate::r#const::{ARRAY, EM_DASH, MEAN, STANDARD_DEVIATION};
    // }
}

pub mod l10n {
    pub const EN: &[&str] = &[include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/ftl/en/attributes.ftl"
    ))];

    pub const RU: &[&str] = &[include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/ftl/ru/attributes.ftl"
    ))];
}

pub mod buttons;
pub mod r#const;
pub mod settings;

#[cfg(feature = "fatty_acids")]
pub mod fatty_acids;

#[cfg(feature = "polars")]
pub mod polars;

pub(crate) mod utils;
