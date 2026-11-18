use serde::{Deserialize, Serialize};

/**
The network protocol used to fetch the resource as identified by the ALPN
Protocol ID.

- <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
- <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/net/http/http_connection_info.cc>
- <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/net/socket/next_proto.cc>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum NetworkErrorProtocol {
    /**
    HTTP/2 over TLS (`"h2"`).
    */
    #[serde(rename = "h2")]
    H2,
    /**
    HTTP/3 (`"h3"`).
    */
    #[serde(rename = "h3")]
    H3,
    /**
    HTTP/0.9 (`"http/0.9"`).
    */
    #[serde(rename = "http/0.9")]
    Http09,
    /**
    HTTP/1.0 (`"http/1.0"`).
    */
    #[serde(rename = "http/1.0")]
    Http10,
    /**
    HTTP/1.1 (`"http/1.1"`).
    */
    #[serde(rename = "http/1.1")]
    Http11,
    /**
    QUIC (`"quic"`).
    */
    #[serde(rename = "quic")]
    Quic,
    /**
    SPDY/3 (`"spdy/3"`).
    */
    #[serde(rename = "spdy/3")]
    Spdy3,
    /**
    Unknown protocol (`"unknown"`).
    */
    #[serde(rename = "unknown")]
    Unknown,
    /**
    Any other protocol string not explicitly enumerated above (such as legacy
    draft QUIC/HTTP/3 strings).
    */
    #[serde(untagged)]
    Other(String),
}
