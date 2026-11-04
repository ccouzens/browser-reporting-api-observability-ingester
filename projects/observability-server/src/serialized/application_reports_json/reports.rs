use serde::{Deserialize, Serialize};

use super::Report;

#[derive(Debug, Deserialize, Serialize)]
pub struct Reports(Vec<Report>);
