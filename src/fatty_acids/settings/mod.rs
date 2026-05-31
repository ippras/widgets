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

pub mod expressions;
