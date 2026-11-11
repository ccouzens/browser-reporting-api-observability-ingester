use super::Report;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Reports(
    // Safari 26 sends reports individually without the wrapping array. Whilst not spec
    // conforming, we should be compatible with it.
    #[serde_as(as = "OneOrMany<_, PreferMany>")] Vec<Report>,
);
