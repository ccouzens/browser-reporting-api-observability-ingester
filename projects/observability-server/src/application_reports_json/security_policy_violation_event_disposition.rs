use serde::{Deserialize, Serialize};

/**
Each policy has a disposition, which is either "enforce" or "report".

Each violation has a disposition, which is the disposition of the policy that
has been violated.

- https://w3c.github.io/webappsec-csp/#policy-disposition
- https://w3c.github.io/webappsec-csp/#violation-disposition
- https://w3c.github.io/webappsec-csp/#enumdef-securitypolicyviolationeventdisposition
- https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-disposition
- https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SecurityPolicyViolationEventDisposition {
    /**
    The policy was enforced (for example, set via the `Content-Security-Policy`
    header).

    - https://w3c.github.io/webappsec-csp/#dom-securitypolicyviolationeventdisposition-enforce
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition
    */
    Enforce,
    /**
    The policy was report-only (for example, set via the
    `Content-Security-Policy-Report-Only` header).

    - https://w3c.github.io/webappsec-csp/#dom-securitypolicyviolationeventdisposition-report
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#disposition
    */
    Report,
}
