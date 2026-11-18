use super::{
    BlockedUrl, ConnectionAllowlistConnection, CspDirective, Disposition, HttpMethod,
    NetworkErrorPhase, NetworkErrorProtocol, NetworkErrorType, PermissionsPolicyFeature,
    StrippedUrl,
};
use serde::{Deserialize, Deserializer, Serialize, de::IntoDeserializer};
use std::net::IpAddr;

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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(
    tag = "type",
    content = "body",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum ReportBody {
    /**
    Connection Allowlist Violation Reports

    Connection allowlist violation reports indicate that a context (such as a
    Document or Worker) attempted to connect to a destination not permitted by
    an active or report-only Connection Allowlist policy.

    Specifications & Documentation:
    - <https://wicg.github.io/connection-allowlists/#reporting>
    - <https://wicg.github.io/connection-allowlists/#dictdef-connectionallowlistviolationreport>
    - <https://developer.mozilla.org/en-US/docs/Web/API/ReportingObserver>
    - <https://developer.chrome.com/blog/connection-allowlists>
    - <https://web.dev/articles/reporting-api>

    Browser Implementations:
    - Chromium (Blink):
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/connection_allowlist_violation_report_body.idl>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/connection_allowlist_violation_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/connection_allowlist_violation_report_body.cc>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/modules/peerconnection/rtc_peer_connection.cc>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/services/network/network_service_network_delegate.cc>
    - Gecko (Firefox): Not implemented
      - <https://github.com/mozilla/standards-positions/issues/1322>
    - WebKit (Safari): Not implemented
      - <https://github.com/WebKit/standards-positions/issues/583>
    */
    ConnectionAllowlist {
        /**
        environment’s creation URL, stripped for use in reports.

        - <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-url>
        - <https://wicg.github.io/connection-allowlists/#reporting>
        */
        url: Option<StrippedUrl>,
        /**
        ConnectionAllowlistViolationReport’s connection is the serialized URL of
        the connection which violated the allowlist.

        If resource URL is a URL, then resource URL, stripped for use in
        reports. Otherwise, resource URL.

        - <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-connection>
        - <https://wicg.github.io/connection-allowlists/#reporting>
        */
        connection: ConnectionAllowlistConnection,
        /**
        ConnectionAllowlistViolationReport’s allowlist is the allowlist which
        was violated.

        A new list containing the result of serializing each pattern in
        allowlist’s allowlist.

        - <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-allowlist>
        - <https://wicg.github.io/connection-allowlists/#connection-allowlists>
        - <https://wicg.github.io/connection-allowlists/#reporting>
        - <https://urlpattern.spec.whatwg.org/>
        */
        allowlist: Option<Vec<String>>,
        /**
        ConnectionAllowlistViolationReport’s disposition is the allowlist’s
        disposition.

        - <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-disposition>
        - <https://wicg.github.io/connection-allowlists/#enumdef-connectionallowlistdisposition>
        - <https://wicg.github.io/connection-allowlists/#connection-allowlists>
        - <https://wicg.github.io/connection-allowlists/#reporting>
        */
        disposition: Disposition,
    },
    /**
    Content Security Policy Violation Reports

    CSP violation reports indicate that a resource or action on a page violated
    an active or report-only Content Security Policy.

    Specifications & Documentation:
    - <https://w3c.github.io/webappsec-csp/#reporting>
    - <https://w3c.github.io/webappsec-csp/#cspviolationreportbody>
    - <https://w3c.github.io/webappsec-csp/#framework-violation>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport>
    - <https://web.dev/articles/csp>
    - <https://web.dev/articles/reporting-api>

    Browser Implementations:
    - Chromium (Blink):
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/csp/csp_violation_report_body.idl>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/csp/csp_violation_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/csp/csp_violation_report_body.cc>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/location_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/location_report_body.cc>
    - Gecko (Firefox):
      - <https://github.com/mozilla-firefox/firefox/blob/5866b3015de8fc803012d8ab800d8c38df1ea5e8/dom/reporting/CSPViolationReportBody.h>
      - <https://github.com/mozilla-firefox/firefox/blob/5866b3015de8fc803012d8ab800d8c38df1ea5e8/dom/reporting/CSPViolationReportBody.cpp>
    - WebKit (Safari):
      - <https://github.com/WebKit/WebKit/blob/2ea5b33be7f0ffde238363fa949ca0eb33aa3565/Source/WebCore/page/csp/CSPViolationReportBody.h>
      - <https://github.com/WebKit/WebKit/blob/2ea5b33be7f0ffde238363fa949ca0eb33aa3565/Source/WebCore/page/csp/CSPViolationReportBody.cpp>
    */
    CspViolation {
        /**
        Each violation has a global object, which is the global object whose
        policy has been violated.

        Each violation has a url which is its global object’s URL.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-documenturl>
        - <https://w3c.github.io/webappsec-csp/#violation-url>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#documenturl>
        */
        #[serde(rename = "documentURL")]
        document_url: StrippedUrl,
        /**
        Each violation has a referrer, which is either null, or a URL. It
        represents the referrer of the resource whose policy was violated.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-referrer>
        - <https://w3c.github.io/webappsec-csp/#violation-referrer>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#referrer>
        */
        #[serde(default, deserialize_with = "empty_string_as_none")]
        referrer: Option<StrippedUrl>,
        /**
        Each violation has a resource, which is either null, "inline", "eval",
        "wasm-eval", "trusted-types-policy", "trusted-types-sink", or a URL. It
        represents the resource which violated the policy.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-blockedurl>
        - <https://w3c.github.io/webappsec-csp/#violation-resource>
        - <https://w3c.github.io/webappsec-csp/#obtain-violation-blocked-uri>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl>
        */
        #[serde(rename = "blockedURL")]
        blocked_url: Option<BlockedUrl>,
        /**
        Each violation has an effective directive which is a non-empty string
        representing the directive whose enforcement caused the violation.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-effectivedirective>
        - <https://w3c.github.io/webappsec-csp/#violation-effective-directive>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#effectivedirective>
        */
        effective_directive: CspDirective,
        /**
        Each violation has a policy, which is the policy that has been
        violated.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-originalpolicy>
        - <https://w3c.github.io/webappsec-csp/#violation-policy>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#originalpolicy>
        */
        original_policy: String,
        /**
        Each violation has a source file, which is either null, or a URL.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-sourcefile>
        - <https://w3c.github.io/webappsec-csp/#violation-source-file>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#sourcefile>
        */
        source_file: Option<StrippedUrl>,
        /**
        Each violation has a sample, which is a string.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-sample>
        - <https://w3c.github.io/webappsec-csp/#violation-sample>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#sample>
        */
        #[serde(default, deserialize_with = "empty_string_as_none")]
        sample: Option<String>,
        /**
        Each violation has a disposition, which is the disposition of the
        policy that has been violated.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-disposition>
        - <https://w3c.github.io/webappsec-csp/#violation-disposition>
        - <https://w3c.github.io/webappsec-csp/#enumdef-securitypolicyviolationeventdisposition>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition>
        */
        disposition: Disposition,
        /**
        Each violation has a status, which is a non-negative integer
        representing the HTTP status code of the resource for which the global
        object was instantiated.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-statuscode>
        - <https://w3c.github.io/webappsec-csp/#violation-status>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#statuscode>
        */
        status_code: u16,
        /**
        Each violation has a line number, which is a non-negative integer.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-linenumber>
        - <https://w3c.github.io/webappsec-csp/#violation-line-number>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#linenumber>
        */
        line_number: Option<u32>,
        /**
        Each violation has a column number, which is a non-negative integer.

        - <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-columnnumber>
        - <https://w3c.github.io/webappsec-csp/#violation-column-number>
        - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#columnnumber>
        */
        column_number: Option<u32>,
    },
    /**
    Deprecation Reports

    Deprecation reports indicate that a page has used a feature or API that is
    expected to be removed in a future version of the browser.

    Specifications & Documentation:
    - <https://wicg.github.io/deprecation-reporting/#deprecation-report>
    - <https://wicg.github.io/deprecation-reporting/#deprecationreportbody>
    - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport>
    - <https://web.dev/articles/reportingobserver>
    - <https://web.dev/articles/reporting-api>

    Browser Implementations:
    - Chromium (Blink):
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/deprecation/deprecation_report_body.idl>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/deprecation/deprecation_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/deprecation/deprecation_report_body.cc>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/deprecation/deprecation.cc>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/location_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/location_report_body.cc>
    - Gecko (Firefox):
      - <https://github.com/mozilla-firefox/firefox/blob/5866b3015de8fc803012d8ab800d8c38df1ea5e8/dom/reporting/DeprecationReportBody.h>
      - <https://github.com/mozilla-firefox/firefox/blob/5866b3015de8fc803012d8ab800d8c38df1ea5e8/dom/reporting/DeprecationReportBody.cpp>
    - WebKit (Safari):
      - <https://github.com/WebKit/WebKit/blob/2ea5b33be7f0ffde238363fa949ca0eb33aa3565/Source/WebCore/Modules/reporting/DeprecationReportBody.h>
      - <https://github.com/WebKit/WebKit/blob/2ea5b33be7f0ffde238363fa949ca0eb33aa3565/Source/WebCore/Modules/reporting/DeprecationReportBody.cpp>
    */
    Deprecation {
        /**
        An implementation-defined string identifying the feature or API that
        will be removed. This string can be used for grouping and counting
        related reports.

        - <https://wicg.github.io/deprecation-reporting/#dom-deprecationreportbody-id>
        - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport#id>
        */
        id: String,
        /**
        A JavaScript Date object (rendered as an ISO 8601 string) indicating
        roughly when the browser version without the specified API will be
        generally available (excluding "beta" or other pre-release channels).
        This value should be used to sort or prioritize warnings. If unknown,
        this field should be null, and the deprecation should be considered low
        priority (removal may not actually occur).

        - <https://wicg.github.io/deprecation-reporting/#dom-deprecationreportbody-anticipatedremoval>
        - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport#anticipatedremoval>
        */
        anticipated_removal: Option<jiff::Timestamp>,
        /**
        A human-readable string with details typically matching what would be
        displayed on the developer console. The message is not guaranteed to be
        unique for a given id (e.g. it may contain additional context on how
        the API was used).

        - <https://wicg.github.io/deprecation-reporting/#dom-deprecationreportbody-message>
        - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport#message>
        */
        message: String,
        /**
        If known, the file which first used the indicated API, or null
        otherwise.

        - <https://wicg.github.io/deprecation-reporting/#dom-deprecationreportbody-sourcefile>
        - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport#sourcefile>
        */
        source_file: Option<StrippedUrl>,
        /**
        If known, the line number in sourceFile where the indicated API was
        first used, or null otherwise.

        - <https://wicg.github.io/deprecation-reporting/#dom-deprecationreportbody-linenumber>
        - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport#linenumber>
        */
        line_number: Option<u32>,
        /**
        If known, the column number in sourceFile where the indicated API was
        first used, or null otherwise.

        - <https://wicg.github.io/deprecation-reporting/#dom-deprecationreportbody-columnnumber>
        - <https://developer.mozilla.org/en-US/docs/Web/API/DeprecationReport#columnnumber>
        */
        column_number: Option<u32>,
    },
    /**
    Network Error Logging Reports

    Network error reports indicate that a network request failed (or succeeded,
    when `success_fraction` is configured) under an active Network Error
    Logging (NEL) policy.

    Specifications & Documentation:
    - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging>

    Browser Implementations:
    - Chromium (Blink / net):
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/net/network_error_logging/network_error_logging_service.cc>
    - Gecko (Firefox): Not implemented
    - WebKit (Safari): Not implemented
    */
    NetworkError {
        /**
        The sampling rate applied to this request.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(rename = "sampling_fraction")]
        sampling_fraction: f64,
        /**
        The elapsed number of milliseconds between the start of the resource
        fetch and when it was completed or aborted by the user agent.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(rename = "elapsed_time")]
        elapsed_time: Option<u64>,
        /**
        If request failed, the phase of its network error. If request
        succeeded, "application".

        - <https://w3c.github.io/network-error-logging/#dfn-type-phase>
        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        phase: NetworkErrorPhase,
        /**
        If request failed, the type of its network error. If request succeeded,
        "ok".

        - <https://w3c.github.io/network-error-logging/#dfn-types>
        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(rename = "type")]
        r#type: NetworkErrorType,
        /**
        The IP address of the server to which the user agent sent the request,
        if available.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(
            rename = "server_ip",
            default,
            deserialize_with = "empty_string_as_none"
        )]
        server_ip: Option<IpAddr>,
        /**
        The network protocol used to fetch the resource as identified by the
        ALPN Protocol ID, if available.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(default, deserialize_with = "empty_string_as_none")]
        protocol: Option<NetworkErrorProtocol>,
        /**
        request's referrer, as determined by the referrer policy associated with
        its client.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(default, deserialize_with = "empty_string_as_none")]
        referrer: Option<StrippedUrl>,
        /**
        request's request method.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        method: Option<HttpMethod>,
        /**
        The status code of the HTTP response, if available. Otherwise, 0.

        - <https://w3c.github.io/network-error-logging/#generate-a-network-error-report>
        - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
        */
        #[serde(rename = "status_code")]
        status_code: Option<u16>,
    },
    /**
    Permissions Policy Violation Reports

    Permissions policy violation reports indicate that some behavior of the
    Document has violated a permissions policy.

    Specifications & Documentation:
    - <https://w3c.github.io/webappsec-permissions-policy/#reporting>
    - <https://w3c.github.io/webappsec-permissions-policy/#permissionspolicyviolationreportbody>
    - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport>
    - <https://web.dev/articles/reporting-api>

    Browser Implementations:
    - Chromium (Blink):
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/permissions_policy_violation_report_body.idl>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/permissions_policy_violation_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/permissions_policy_violation_report_body.cc>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/location_report_body.h>
      - <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/third_party/blink/renderer/core/frame/location_report_body.cc>
    - Gecko (Firefox): Not implemented
    - WebKit (Safari): Not implemented
    */
    PermissionsPolicyViolation {
        /**
        The string identifying the policy-controlled feature whose policy has
        been violated. This string can be used for grouping and counting
        related reports.

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-featureid>
        - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#featureid>
        */
        #[serde(alias = "policyId")]
        feature_id: PermissionsPolicyFeature,
        /**
        If known, the file where the violation occured, or null otherwise.

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-sourcefile>
        - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#sourcefile>
        */
        source_file: Option<StrippedUrl>,
        /**
        If known, the line number in sourceFile where the violation occured, or
        null otherwise.

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-linenumber>
        - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#linenumber>
        */
        line_number: Option<u32>,
        /**
        If known, the column number in sourceFile where the violation occured,
        or null otherwise.

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-columnnumber>
        - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#columnnumber>
        */
        column_number: Option<u32>,
        /**
        A string indicating whether the violated permissions policy was enforced
        in this case. disposition will be set to "enforce" if the policy was
        enforced, or "report" if the violation resulted only in this report
        being generated (with no further action taken by the user agent in
        response to the violation).

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-disposition>
        - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#disposition>
        */
        disposition: Disposition,
        /**
        For reports of potential violations, which can be attributed to a
        specific <iframe> element, the value of the allow attribute of that
        element, or omitted otherwise.

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-allowattribute>
        */
        allow_attribute: Option<String>,
        /**
        For reports of potential violations, which can be attributed to a
        specific <iframe> element, the value of the src attribute of that
        element, or omitted otherwise.

        - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-srcattribute>
        */
        src_attribute: Option<String>,
        /**
        A string containing a human-readable description of the violation
        (Blink extension, not in the W3C specification).

        - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#message>
        */
        message: Option<String>,
    },
    #[serde(untagged)]
    UnknownViolation {
        r#type: String,
        body: serde_json::Value,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use url::Url;

    #[test]
    fn test_deserialize_connection_allowlist_report() {
        let json = json!({
            "type": "connection-allowlist",
            "body": {
                "url": "https://happen-land-comp-basically.trycloudflare.com/",
                "connection": "https://www.gstatic.com/devrel-devsite/prod/v097103d8ccd7b257790809ea4cc5e5fee6daa6e24890a4af388296e58364bc31/chrome/images/lockup.svg",
                "allowlist": ["https://happen-land-comp-basically.trycloudflare.com/*"],
                "disposition": "report"
            }
        });

        let report_body: ReportBody = serde_json::from_value(json).unwrap();
        assert_eq!(
            report_body,
            ReportBody::ConnectionAllowlist {
                url: Some(StrippedUrl::Url(
                    Url::parse("https://happen-land-comp-basically.trycloudflare.com/").unwrap()
                )),
                connection: ConnectionAllowlistConnection::Url(StrippedUrl::Url(
                    Url::parse(
                        "https://www.gstatic.com/devrel-devsite/prod/v097103d8ccd7b257790809ea4cc5e5fee6daa6e24890a4af388296e58364bc31/chrome/images/lockup.svg"
                    )
                    .unwrap()
                )),
                allowlist: Some(vec![
                    "https://happen-land-comp-basically.trycloudflare.com/*"
                        .to_string(),
                ]),
                disposition: Disposition::Report,
            }
        );
    }

    #[test]
    fn test_deserialize_csp_violation_report() {
        let json = json!({
            "type": "csp-violation",
            "body": {
                "documentURL": "https://happen-land-comp-basically.trycloudflare.com/",
                "referrer": "https://example.com/",
                "blockedURL": "data",
                "effectiveDirective": "img-src",
                "originalPolicy": "img-src 'self' https://www.gstatic.com; report-to default",
                "sourceFile": "https://happen-land-comp-basically.trycloudflare.com/assets/index-BIAicTOC.js",
                "sample": "console.log('violation')",
                "disposition": "enforce",
                "statusCode": 200,
                "lineNumber": 3,
                "columnNumber": 1
            }
        });

        let report_body: ReportBody = serde_json::from_value(json).unwrap();
        assert_eq!(
            report_body,
            ReportBody::CspViolation {
                document_url: StrippedUrl::Url(
                    Url::parse("https://happen-land-comp-basically.trycloudflare.com/").unwrap()
                ),
                referrer: Some(StrippedUrl::Url(
                    Url::parse("https://example.com/").unwrap()
                )),
                blocked_url: Some(BlockedUrl::Url(StrippedUrl::Scheme("data".to_string()))),
                effective_directive: CspDirective::ImgSrc,
                original_policy: "img-src 'self' https://www.gstatic.com; report-to default"
                    .to_string(),
                source_file: Some(StrippedUrl::Url(
                    Url::parse(
                        "https://happen-land-comp-basically.trycloudflare.com/assets/index-BIAicTOC.js"
                    )
                    .unwrap()
                )),
                sample: Some("console.log('violation')".to_string()),
                disposition: Disposition::Enforce,
                status_code: 200,
                line_number: Some(3),
                column_number: Some(1),
            }
        );
    }

    #[test]
    fn test_deserialize_deprecation_report() {
        let json = json!({
            "type": "deprecation",
            "body": {
                "id": "UnloadHandler",
                "anticipatedRemoval": "2027-01-01T00:00:00Z",
                "message": "Unload event listeners are deprecated and will be removed.",
                "sourceFile": "https://happen-land-comp-basically.trycloudflare.com/assets/index-BIAicTOC.js",
                "lineNumber": 10,
                "columnNumber": 1
            }
        });

        let report_body: ReportBody = serde_json::from_value(json).unwrap();
        assert_eq!(
            report_body,
            ReportBody::Deprecation {
                id: "UnloadHandler".to_string(),
                anticipated_removal: Some("2027-01-01T00:00:00Z".parse().unwrap()),
                message: "Unload event listeners are deprecated and will be removed.".to_string(),
                source_file: Some(StrippedUrl::Url(
                    Url::parse(
                        "https://happen-land-comp-basically.trycloudflare.com/assets/index-BIAicTOC.js"
                    )
                    .unwrap()
                )),
                line_number: Some(10),
                column_number: Some(1),
            }
        );
    }

    #[test]
    fn test_deserialize_network_error_report() {
        let json = json!({
            "type": "network-error",
            "body": {
                "sampling_fraction": 1.0,
                "elapsed_time": 106,
                "phase": "application",
                "type": "http.error",
                "server_ip": "104.16.231.132",
                "protocol": "h2",
                "referrer": "https://brunswick-front-manufacturer-youth.trycloudflare.com/",
                "method": "GET",
                "status_code": 404
            }
        });

        let report_body: ReportBody = serde_json::from_value(json).unwrap();
        assert_eq!(
            report_body,
            ReportBody::NetworkError {
                sampling_fraction: 1.0,
                elapsed_time: Some(106),
                phase: NetworkErrorPhase::Application,
                r#type: NetworkErrorType::HttpError,
                server_ip: Some("104.16.231.132".parse().unwrap()),
                protocol: Some(NetworkErrorProtocol::H2),
                referrer: Some(StrippedUrl::Url(
                    Url::parse("https://brunswick-front-manufacturer-youth.trycloudflare.com/")
                        .unwrap()
                )),
                method: Some(HttpMethod::Get),
                status_code: Some(404),
            }
        );
    }

    #[test]
    fn test_deserialize_permissions_policy_violation_report() {
        let json = json!({
            "type": "permissions-policy-violation",
            "body": {
                "featureId": "geolocation",
                "sourceFile": "https://happen-land-comp-basically.trycloudflare.com/assets/index-BIAicTOC.js",
                "lineNumber": 44,
                "columnNumber": 29,
                "disposition": "enforce",
                "allowAttribute": "geolocation 'none'",
                "srcAttribute": "https://example.com/embed.html",
                "message": "Permissions policy violation: geolocation is not allowed in this document."
            }
        });

        let report_body: ReportBody = serde_json::from_value(json).unwrap();
        assert_eq!(
            report_body,
            ReportBody::PermissionsPolicyViolation {
                feature_id: PermissionsPolicyFeature::Geolocation,
                source_file: Some(StrippedUrl::Url(
                    Url::parse(
                        "https://happen-land-comp-basically.trycloudflare.com/assets/index-BIAicTOC.js"
                    )
                    .unwrap()
                )),
                line_number: Some(44),
                column_number: Some(29),
                disposition: Disposition::Enforce,
                allow_attribute: Some("geolocation 'none'".to_string()),
                src_attribute: Some("https://example.com/embed.html".to_string()),
                message: Some(
                    "Permissions policy violation: geolocation is not allowed in this document."
                        .to_string()
                ),
            }
        );
    }
}
