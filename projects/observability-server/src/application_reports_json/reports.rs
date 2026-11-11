use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

use super::Report;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Reports {
    Multiple(Vec<Report>),
    // Safari 26 sends a single report instead of an array. I don't believe
    // this to be spec compliant.
    Single(Box<Report>),
}

impl Serialize for Reports {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Multiple(reports) => {
                let mut seq = serializer.serialize_seq(Some(reports.len()))?;
                for report in reports {
                    seq.serialize_element(report)?;
                }
                seq.end()
            }
            Self::Single(report) => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element(report)?;
                seq.end()
            }
        }
    }
}
