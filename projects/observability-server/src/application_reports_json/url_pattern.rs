use serde::{Deserialize, Deserializer, Serialize, Serializer};
use urlpattern::{Error, UrlPatternInit, regexp::RegExp};

fn parse_url_pattern<R: RegExp>(pattern: &str) -> Result<urlpattern::UrlPattern<R>, Error> {
    let init = UrlPatternInit::parse_constructor_string::<R>(pattern, None)?;
    urlpattern::UrlPattern::<R>::parse(init, Default::default())
}

/**
A serialized URLPattern from a report, validated using the `urlpattern` crate.

- <https://urlpattern.spec.whatwg.org/>
- <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-allowlist>
*/
#[derive(Debug)]
pub struct UrlPattern {
    source: String,
    pattern: urlpattern::UrlPattern,
}

impl UrlPattern {
    /**
    Parses a URLPattern constructor string into a [`UrlPattern`].
    */
    pub fn parse(source: impl Into<String>) -> Result<Self, Error> {
        let source = source.into();
        let pattern = parse_url_pattern(&source)?;
        Ok(Self { source, pattern })
    }

    /**
    Returns the underlying [`urlpattern::UrlPattern`].
    */
    pub fn as_url_pattern(&self) -> &urlpattern::UrlPattern {
        &self.pattern
    }

    /**
    Returns the original URLPattern string.
    */
    pub fn as_str(&self) -> &str {
        &self.source
    }
}

impl PartialEq for UrlPattern {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for UrlPattern {}

impl<'de> Deserialize<'de> for UrlPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let source = String::deserialize(deserializer)?;
        Self::parse(source).map_err(serde::de::Error::custom)
    }
}

impl Serialize for UrlPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.source)
    }
}
