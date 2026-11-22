use serde::{Deserialize, Serialize};
use user_agent_classifier::UserAgent;

use super::{ReportBody, StrippedUrl};

/**
2.1.3. Reports

A report is a collection of arbitrary data which the user agent is expected to
deliver to a specified endpoint.

- <https://w3c.github.io/reporting/#concept-reports>
- <https://www.w3.org/TR/reporting-1/#concept-reports>
- <https://w3c.github.io/reporting/#serialize-reports>
- <https://www.w3.org/TR/reporting-1/#serialize-reports>
- <https://developer.mozilla.org/en-US/docs/Web/API/Report>
- <https://web.dev/articles/reporting-api>
- <https://web.dev/articles/reportingobserver>
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    /**
    The number of milliseconds between report’s timestamp and the current time.

    - <https://w3c.github.io/reporting/#serialize-reports>
    - <https://www.w3.org/TR/reporting-1/#serialize-reports>
    */
    #[serde(default)]
    age: Option<u32>,
    /**
    Each report has a url, which is typically the address of the Document or
    Worker from which the report was generated.

    - <https://w3c.github.io/reporting/#report-url>
    - <https://www.w3.org/TR/reporting-1/#report-url>
    - <https://developer.mozilla.org/en-US/docs/Web/API/Report/url>
    */
    url: StrippedUrl,
    /**
    Each report has a user agent, which is the value of the User-Agent header of
    the request from which the report was generated.

    Note: The user agent of a report represents the User-Agent sent by the
    browser for the page which generated the report. This is potentially
    distinct from the User-Agent sent in the HTTP headers when uploading the
    report to a collector — for instance, where the browser has chosen to use a
    non-default User-Agent string such as the "request desktop site" feature.

    - <https://w3c.github.io/reporting/#report-user-agent>
    - <https://www.w3.org/TR/reporting-1/#report-user-agent>
    */
    #[serde(default)]
    user_agent: Option<UserAgent>,
    /**
    Each report has a body, which is either null or an object which can be
    serialized into a JSON text. The fields contained in a report’s body are
    determined by the report’s type.

    - <https://w3c.github.io/reporting/#report-body>
    - <https://www.w3.org/TR/reporting-1/#report-body>
    - <https://developer.mozilla.org/en-US/docs/Web/API/Report/body>
    */
    #[serde(flatten)]
    body: ReportBody,
}
