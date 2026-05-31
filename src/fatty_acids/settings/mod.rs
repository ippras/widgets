pub use self::expressions::Expressions;

use serde::{Deserialize, Serialize};

/// Settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub threshold: bool,
    pub sort: bool,
    pub filter: bool,
    pub expressions: Expressions,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            threshold: false,
            sort: false,
            filter: false,
            expressions: Expressions::new(),
        }
    }
}

pub mod expressions;
