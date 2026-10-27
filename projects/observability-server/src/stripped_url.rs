use serde::{Deserialize, Serialize};
use url::Url;

/**
5.4. Strip URL for use in reports
Given a URL url, this algorithm returns a string representing the URL for use in violation reports:

1. If url’s scheme is not an HTTP(S) scheme, then return url’s scheme.
2. Set url’s fragment to the empty string.
3. Set url’s username to the empty string.
4. Set url’s password to the empty string.
5. Return the result of executing the URL serializer on url.

https://w3c.github.io/webappsec-csp/#strip-url-for-use-in-reports
*/
// need to add manual deserializers, serializers
#[derive(Debug, Deserialize, Serialize)]
pub struct StrippedUrl {
    pub scheme: String,
    pub url: Option<Url>,
}
