use serde::{Deserialize, Serialize, Serializer};

use super::Report;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Reports {
    Multiple(Vec<Report>),
    // Safari 26 sends a single report instead of an array. I don't believe
    // this to be spec compliant.
    Single(Box<Report>),
}
impl Reports {
    pub fn as_slice(&self) -> &[Report] {
        match self {
            Self::Multiple(reports) => reports.as_slice(),
            Self::Single(report) => std::slice::from_ref(report),
        }
    }
}

impl Serialize for Reports {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_slice().serialize(serializer)
    }
}
