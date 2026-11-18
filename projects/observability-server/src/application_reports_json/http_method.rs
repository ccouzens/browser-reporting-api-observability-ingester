use serde::{Deserialize, Serialize};

/**
An HTTP request method.

- <https://www.rfc-editor.org/rfc/rfc9110#section-9.3>
- <https://www.rfc-editor.org/rfc/rfc5789#section-2>
- <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /**
    Establish a tunnel to the server identified by the target resource.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.6>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/CONNECT>
    */
    Connect,
    /**
    Delete the specified resource.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.5>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/DELETE>
    */
    Delete,
    /**
    Request a representation of the specified resource.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.1>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/GET>
    */
    Get,
    /**
    Ask for a response identical to a `GET` request, but without the response
    body.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.2>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/HEAD>
    */
    Head,
    /**
    Describe the communication options for the target resource.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.7>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/OPTIONS>
    */
    Options,
    /**
    Apply partial modifications to a resource.

    - <https://www.rfc-editor.org/rfc/rfc5789#section-2>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PATCH>
    */
    Patch,
    /**
    Submit an entity to the specified resource.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.3>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/POST>
    */
    Post,
    /**
    Replace all current representations of the target resource with the request
    content.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.3.4>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PUT>
    */
    Put,
    /**
    Any other HTTP method token not explicitly enumerated above.

    - <https://www.rfc-editor.org/rfc/rfc9110#section-9.1>
    */
    #[serde(untagged)]
    Other(String),
}
