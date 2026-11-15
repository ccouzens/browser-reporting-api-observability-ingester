use super::StrippedUrl;
use serde::{Deserialize, Serialize};

/**
Each violation has a resource, which is either null, "inline", "eval",
"wasm-eval", "trusted-types-policy", "trusted-types-sink", or a URL. It
represents the resource which violated the policy.

Given a violation’s resource resource, this algorithm returns a string, to be
used as the blocked URI field for violation reports:

1. Assert: resource is a URL or a string.
2. If resource is a URL, return the result of executing § 5.4 Strip URL for use
   in reports on resource.
3. Return resource.

- https://w3c.github.io/webappsec-csp/#violation-resource
- https://w3c.github.io/webappsec-csp/#obtain-violation-blocked-uri
- https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-blockedurl
- https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockedUrl {
    /**
    An inline resource used when `'unsafe-inline'` is not specified in the CSP
    (for example, an inline `<script>` or `style` attribute).

    - https://w3c.github.io/webappsec-csp/#violation-resource
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
    */
    Inline,
    /**
    `eval()` used when `'unsafe-eval'` is not specified in the CSP.

    - https://w3c.github.io/webappsec-csp/#violation-resource
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
    */
    Eval,
    /**
    Wasm evaluation used when `'wasm-unsafe-eval'` is not specified in the CSP.

    - https://w3c.github.io/webappsec-csp/#violation-resource
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
    */
    WasmEval,
    /**
    Violation of the `trusted-types` CSP directive, for example creating a
    `TrustedTypePolicy` with `window.trustedTypes.createPolicy()` using a name
    not listed in the CSP `trusted-types` directive.

    - https://w3c.github.io/webappsec-csp/#violation-resource
    - https://w3c.github.io/trusted-types/dist/spec/#trusted-types-csp-directive
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
    */
    TrustedTypesPolicy,
    /**
    Violation of the `require-trusted-types-for` CSP directive, where data was
    passed to a DOM XSS sink (such as `Element.innerHTML`) without first being
    sanitized by a `TrustedTypePolicy`.

    - https://w3c.github.io/webappsec-csp/#violation-resource
    - https://w3c.github.io/trusted-types/dist/spec/#require-trusted-types-for-csp-directive
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
    */
    TrustedTypesSink,
    /**
    If `resource` is a URL, the result of executing § 5.4 Strip URL for use in
    reports on `resource`.

    - https://w3c.github.io/webappsec-csp/#obtain-violation-blocked-uri
    - https://w3c.github.io/webappsec-csp/#strip-url-for-use-in-reports
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#blockedurl
    */
    #[serde(untagged)]
    Url(StrippedUrl),
}
