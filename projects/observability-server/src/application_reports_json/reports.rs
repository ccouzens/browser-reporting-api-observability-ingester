use serde::{Deserialize, Serialize};

use super::Report;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Reports {
    Multiple(Vec<Report>),
    // Safari 26 sends a single report instead of an array. I don't believe
    // this to be spec compliant.
    Single(Report),
}
