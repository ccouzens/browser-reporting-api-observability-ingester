use crate::StrippedUrl;
use serde::{Deserialize, Serialize};

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
        referrer: Option<StrippedUrl>,
    },
    #[serde(other)]
    UnknownViolation,
}
