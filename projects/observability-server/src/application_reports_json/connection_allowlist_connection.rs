use super::StrippedUrl;
use serde::{Deserialize, Serialize};

/**
ConnectionAllowlistViolationReport’s connection is the serialized URL of the
connection which violated the allowlist.

To report a violation given a URL or the string "webrtc" (resource URL), an
environment (environment), and a connection allowlist (allowlist):

1. If allowlist’s reporting endpoint is null, return.
2. Let violation be a new ConnectionAllowlistViolationReport, initialized as
   follows:
   - connection: If resource URL is a URL, then resource URL, stripped for use
     in reports. Otherwise, resource URL.

- <https://wicg.github.io/connection-allowlists/#reporting>
- <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-connection>
- <https://developer.chrome.com/blog/connection-allowlists>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionAllowlistConnection {
    /**
    A WebRTC connection attempt when `webrtc` is `block` (the default unless
    `webrtc=allow` is specified in the `Connection-Allowlist` header).

    - <https://wicg.github.io/connection-allowlists/#connection-allowlists>
    - <https://wicg.github.io/connection-allowlists/#reporting>
    */
    Webrtc,
    /**
    If `resource URL` is a URL, then `resource URL`, stripped for use in
    reports.

    - <https://wicg.github.io/connection-allowlists/#reporting>
    - <https://w3c.github.io/reporting/#strip-url-for-use-in-reports>
    */
    #[serde(untagged)]
    Url(StrippedUrl),
}
