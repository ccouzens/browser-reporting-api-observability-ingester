use serde::{Deserialize, Serialize};

/**
Whether the violated policy was enforced or report-only (`"enforce"` or
`"report"`).

Content Security Policy:
- Each policy has a disposition, which is either "enforce" or "report".
- Each violation has a disposition, which is the disposition of the policy that
  has been violated.
- <https://w3c.github.io/webappsec-csp/#policy-disposition>
- <https://w3c.github.io/webappsec-csp/#violation-disposition>
- <https://w3c.github.io/webappsec-csp/#enumdef-securitypolicyviolationeventdisposition>
- <https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-disposition>
- <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition>
- <https://web.dev/articles/csp>

Connection Allowlists:
- A Connection Allowlist represents the set of URL patterns to which a given
  context is allowed to connect. It is a struct with the following items:
  - disposition, which is either "enforce" or "report".
- ConnectionAllowlistViolationReport’s disposition is the allowlist’s
  disposition.
- <https://wicg.github.io/connection-allowlists/#connection-allowlists>
- <https://wicg.github.io/connection-allowlists/#reporting>
- <https://wicg.github.io/connection-allowlists/#enumdef-connectionallowlistdisposition>
- <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistviolationreport-disposition>
- <https://developer.chrome.com/blog/connection-allowlists>

Permissions Policy:
- A string indicating whether the violated permissions policy was enforced in
  this case. disposition will be set to "enforce" if the policy was enforced, or
  "report" if the violation resulted only in this report being generated.
- <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-disposition>
- <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#disposition>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Disposition {
    /**
    The policy was enforced (for example, set via the `Content-Security-Policy`,
    `Connection-Allowlist`, or `Permissions-Policy` header).

    - <https://w3c.github.io/webappsec-csp/#dom-securitypolicyviolationeventdisposition-enforce>
    - <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistdisposition-enforce>
    - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-disposition>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition>
    - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#disposition>
    */
    Enforce,
    /**
    The policy was report-only (for example, set via the
    `Content-Security-Policy-Report-Only`, `Connection-Allowlist-Report-Only`,
    or `Permissions-Policy-Report-Only` header).

    - <https://w3c.github.io/webappsec-csp/#dom-securitypolicyviolationeventdisposition-report>
    - <https://wicg.github.io/connection-allowlists/#dom-connectionallowlistdisposition-report>
    - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-disposition>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition>
    - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#disposition>
    */
    Report,
}
