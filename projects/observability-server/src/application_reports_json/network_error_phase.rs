use serde::{Deserialize, Serialize};

/**
The phase of a network request in which a network error occurred (or
`"application"` if the request succeeded).

- <https://w3c.github.io/network-error-logging/#dfn-type-phase>
- <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
- <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
- <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/net/network_error_logging/network_error_logging_service.cc>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkErrorPhase {
    /**
    The error occurred during application-level processing, or the request
    succeeded.
    */
    Application,
    /**
    The error occurred while establishing a transport connection (TCP or TLS).
    */
    Connection,
    /**
    The error occurred during DNS resolution.
    */
    Dns,
    /**
    The error occurred while processing a Signed Exchange (Chromium extension).
    */
    Sxg,
    /**
    Any other phase string not explicitly enumerated above.
    */
    #[serde(untagged)]
    Other(String),
}
