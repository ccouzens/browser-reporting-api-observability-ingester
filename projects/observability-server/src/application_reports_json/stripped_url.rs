use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use url::Url;

/**
5.4. Strip URL for use in reports

Given a URL url, this algorithm returns a string representing the URL for use in
violation reports:

1. If url’s scheme is not an HTTP(S) scheme, then return url’s scheme.
2. Set url’s fragment to the empty string.
3. Set url’s username to the empty string.
4. Set url’s password to the empty string.
5. Return the result of executing the URL serializer on url.

- <https://w3c.github.io/webappsec-csp/#strip-url-for-use-in-reports>
- <https://w3c.github.io/reporting/#strip-url-for-use-in-reports>
- <https://www.w3.org/TR/reporting-1/#strip-url-for-use-in-reports>
*/
#[derive(Debug, PartialEq, Eq)]
pub enum StrippedUrl {
    Scheme(String),
    Url(Url),
}

impl<'de> Deserialize<'de> for StrippedUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with(|c: char| c.is_ascii_alphabetic())
            && s.chars()
                .skip(1)
                .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '+' | '-' | '.'))
        {
            return Ok(Self::Scheme(s.to_ascii_lowercase()));
        }
        match Url::parse(&s) {
            Ok(url) => Ok(Self::Url(url)),
            _ => Err(D::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"a https url or other scheme after the Strip URL steps",
            )),
        }
    }
}

impl Serialize for StrippedUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Scheme(scheme) => serializer.serialize_str(scheme),
            Self::Url(url) => serializer.serialize_str(url.as_str()),
        }
    }
}

impl StrippedUrl {
    pub fn scheme(&self) -> &str {
        match self {
            Self::Scheme(scheme) => scheme.as_ref(),
            Self::Url(url) => url.scheme(),
        }
    }

    pub fn url(&self) -> Option<&Url> {
        match self {
            Self::Scheme(_) => None,
            Self::Url(url) => Some(url),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_deserialize_https_url() {
        assert_eq!(
            serde_json::from_str::<StrippedUrl>(r#""https://example.com/api""#).unwrap(),
            StrippedUrl::Url(Url::parse("https://example.com/api").unwrap())
        );
    }

    #[test]
    fn test_deserialize_http_url() {
        assert_eq!(
            serde_json::from_str::<StrippedUrl>(r#""http://example.com/api""#).unwrap(),
            StrippedUrl::Url(Url::parse("http://example.com/api").unwrap())
        );
    }

    #[test]
    fn test_deserialize_data_scheme() {
        assert_eq!(
            serde_json::from_str::<StrippedUrl>(r#""data""#).unwrap(),
            StrippedUrl::Scheme("data".to_string())
        );
    }

    #[test]
    fn test_deserialize_nonsense() {
        assert_matches!(
            serde_json::from_str::<StrippedUrl>(r#""0123""#),
            Result::<StrippedUrl, _>::Err(_)
        );
    }
}
