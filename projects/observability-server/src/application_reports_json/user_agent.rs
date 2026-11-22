mod parsed_user_agent;
mod reported_os;
mod tokenizer;

pub use parsed_user_agent::ParsedUserAgent;
pub use reported_os::ReportedOs;

use serde::{Deserialize, Serialize};

/**
A raw `User-Agent` header value preserved verbatim for JSON serialization and
logging, with lazy parsing into bounded observability enums.

- <https://w3c.github.io/reporting/#report-user-agent>
- <https://www.w3.org/TR/reporting-1/#report-user-agent>
*/
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserAgent(String);

impl UserAgent {
    /**
    Returns the raw, unmodified `User-Agent` string.
    */
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /**
    Parses the `User-Agent` string into bounded observability fields suitable
    for Prometheus labels and structured logs.
    */
    pub fn parse(&self) -> ParsedUserAgent<'_> {
        ParsedUserAgent::parse(&self.0)
    }
}

impl AsRef<str> for UserAgent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for UserAgent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for UserAgent {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_round_trip() {
        let raw = r#""Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36""#;
        let ua: UserAgent = serde_json::from_str(raw).unwrap();
        assert_eq!(
            ua.as_str(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36"
        );
        assert_eq!(serde_json::to_string(&ua).unwrap(), raw);
    }
}
