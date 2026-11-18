use serde::{Deserialize, Serialize};

/**
The Fetch request destination of a resource blocked or reported by an Integrity
Policy.

- <https://w3c.github.io/webappsec-subresource-integrity/#destination>
- <https://fetch.spec.whatwg.org/#requestdestination>
- <https://developer.mozilla.org/en-US/docs/Web/API/Request/destination>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestDestination {
    /**
    A script resource (e.g., `<script>`).
    */
    Script,
    /**
    A stylesheet resource (e.g., `<link rel="stylesheet">`).
    */
    Style,
    /**
    Any other destination string not explicitly enumerated above.
    */
    #[serde(untagged)]
    Other(String),
}
