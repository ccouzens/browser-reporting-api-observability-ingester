use super::Report;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Reports(#[serde_as(as = "OneOrMany<_, PreferMany>")] Vec<Report>);
