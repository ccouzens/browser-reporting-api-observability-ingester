use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use user_agent_classifier::{Classification, parse};
use yoke::{Yoke, Yokeable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Yokeable)]
struct YokedClassification<'a>(Classification<'a>);

/**
A `User-Agent` header value that preserves the original string for JSON

- <https://w3c.github.io/reporting/#report-user-agent>
- <https://www.w3.org/TR/reporting-1/#report-user-agent>
*/
pub struct UserAgent(Yoke<YokedClassification<'static>, String>);

impl UserAgent {
    /**
    Creates a new [`UserAgent`] and immediately parses its [`Classification`].
    */
    pub fn new(raw: impl Into<String>) -> Self {
        Self(Yoke::attach_to_cart(raw.into(), |s| {
            YokedClassification(parse(s))
        }))
    }

    /**
    Returns the raw, unmodified `User-Agent` string.
    */
    pub fn as_str(&self) -> &str {
        self.0.backing_cart()
    }

    /**
    Returns the pre-parsed [`Classification`] for this `User-Agent`.
    */
    pub fn classification(&self) -> &Classification<'_> {
        &self.0.get().0
    }
}

impl PartialEq for UserAgent {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for UserAgent {}

impl fmt::Debug for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserAgent")
            .field("raw", &self.as_str())
            .field("classification", self.classification())
            .finish()
    }
}

impl AsRef<str> for UserAgent {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for UserAgent {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UserAgent {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for UserAgent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Ok(Self::new(raw))
    }
}

impl Serialize for UserAgent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use user_agent_classifier::{Browser, Engine, ReportedPlatform};

    use super::*;

    #[test]
    fn test_serde_round_trip_and_eager_classification() {
        let raw_json = r#""Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36""#;
        let ua: UserAgent = serde_json::from_str(raw_json).unwrap();
        assert_eq!(
            ua.as_str(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36"
        );
        assert_eq!(
            *ua.classification(),
            Classification {
                is_mobile: false,
                is_webview: false,
                is_bot: false,
                reported_platform: ReportedPlatform::Linux,
                engine: Engine::Blink { major: 156 },
                browser: Some(Browser {
                    name: "Chrome",
                    version: Some((156, 0)),
                }),
            }
        );
        assert_eq!(serde_json::to_string(&ua).unwrap(), raw_json);
    }
}
