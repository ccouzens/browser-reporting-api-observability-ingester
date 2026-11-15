use super::StrippedUrl;
use serde::{Deserialize, Deserializer, Serialize, de::IntoDeserializer};

fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "body")]
pub enum ReportBody {
    /**
    Content Security Policy

    https://w3c.github.io/webappsec-csp/#reporting
    */
    #[serde(rename = "csp-violation")]
    CspViolation {
        /**
        Each violation has a global object, which is the global object whose policy has been violated.

        Each violation has a url which is its global object’s URL.
        https://w3c.github.io/webappsec-csp/#framework-violation
        */
        #[serde(rename = "documentURL")]
        document_url: StrippedUrl,
        /**
        Each violation has a referrer, which is either null, or a URL. It represents the referrer of the resource whose policy was violated.

        https://w3c.github.io/webappsec-csp/#framework-violation
        */
        #[serde(deserialize_with = "empty_string_as_none")]
        referrer: Option<StrippedUrl>,
    },
    #[serde(untagged)]
    UnknownViolation {
        r#type: String,
        body: serde_json::Value,
    },
}
