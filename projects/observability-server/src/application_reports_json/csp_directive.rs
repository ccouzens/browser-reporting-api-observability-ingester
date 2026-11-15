use serde::{Deserialize, Serialize};

/**
Each violation has an effective directive which is a non-empty string
representing the directive whose enforcement caused the violation.

- https://w3c.github.io/webappsec-csp/#violation-effective-directive
- https://w3c.github.io/webappsec-csp/#csp-directives
- https://w3c.github.io/webappsec-csp/#effective-directive-for-a-request
- https://w3c.github.io/webappsec-csp/#effective-directive-for-inline-check
- https://w3c.github.io/webappsec-csp/#dom-cspviolationreportbody-effectivedirective
- https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#effectivedirective
- https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy#directives
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CspDirective {
    /**
    The `base-uri` directive restricts the URLs which can be used in a
    Document's `base` element.

    - https://w3c.github.io/webappsec-csp/#directive-base-uri
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/base-uri
    */
    BaseUri,
    /**
    The `connect-src` directive restricts the URLs which can be loaded using
    script interfaces.

    - https://w3c.github.io/webappsec-csp/#directive-connect-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/connect-src
    */
    ConnectSrc,
    /**
    The `fenced-frame-src` directive restricts the URLs which may be loaded
    into a `fencedframe` element's fenced navigable.

    - https://wicg.github.io/fenced-frame/#csp-fenced-frame-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/fenced-frame-src
    */
    FencedFrameSrc,
    /**
    The `font-src` directive restricts the URLs from which font resources may
    be loaded.

    - https://w3c.github.io/webappsec-csp/#directive-font-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/font-src
    */
    FontSrc,
    /**
    The `form-action` directive restricts the URLs which can be used as the
    target of a form submission from a given context.

    - https://w3c.github.io/webappsec-csp/#directive-form-action
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/form-action
    */
    FormAction,
    /**
    The `frame-ancestors` directive restricts the URLs which can embed the
    resource using `frame`, `iframe`, `object`, `embed`, or `applet` element.

    - https://w3c.github.io/webappsec-csp/#directive-frame-ancestors
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/frame-ancestors
    */
    FrameAncestors,
    /**
    The `frame-src` directive restricts the URLs which may be loaded into child
    navigables.

    - https://w3c.github.io/webappsec-csp/#directive-frame-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/frame-src
    */
    FrameSrc,
    /**
    The `img-src` directive restricts the URLs from which image resources may
    be loaded.

    - https://w3c.github.io/webappsec-csp/#directive-img-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/img-src
    */
    ImgSrc,
    /**
    The `manifest-src` directive restricts the URLs from which application
    manifests may be loaded.

    - https://w3c.github.io/webappsec-csp/#directive-manifest-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/manifest-src
    */
    ManifestSrc,
    /**
    The `media-src` directive restricts the URLs from which video, audio, and
    associated text track resources may be loaded.

    - https://w3c.github.io/webappsec-csp/#directive-media-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/media-src
    */
    MediaSrc,
    /**
    The `object-src` directive restricts the URLs from which plugin content may
    be loaded.

    - https://w3c.github.io/webappsec-csp/#directive-object-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/object-src
    */
    ObjectSrc,
    /**
    The `require-trusted-types-for` directive instructs the user agent to
    control the data passed to DOM XSS sink functions.

    - https://w3c.github.io/trusted-types/dist/spec/#require-trusted-types-for-csp-directive
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/require-trusted-types-for
    */
    RequireTrustedTypesFor,
    /**
    The `script-src` directive restricts the locations from which scripts may
    be executed.

    - https://w3c.github.io/webappsec-csp/#directive-script-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src
    */
    ScriptSrc,
    /**
    The `script-src-attr` directive applies to event handlers and, if present,
    it will override the `script-src` directive for relevant checks.

    - https://w3c.github.io/webappsec-csp/#directive-script-src-attr
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src-attr
    */
    ScriptSrcAttr,
    /**
    The `script-src-elem` directive applies to all script requests and script
    blocks.

    - https://w3c.github.io/webappsec-csp/#directive-script-src-elem
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src-elem
    */
    ScriptSrcElem,
    /**
    The `style-src` directive restricts the locations from which style may be
    applied to a Document.

    - https://w3c.github.io/webappsec-csp/#directive-style-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src
    */
    StyleSrc,
    /**
    The `style-src-attr` directive applies to style attributes and, if present,
    it will override the `style-src` directive for relevant checks.

    - https://w3c.github.io/webappsec-csp/#directive-style-src-attr
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src-attr
    */
    StyleSrcAttr,
    /**
    The `style-src-elem` directive applies to all style requests and style
    blocks.

    - https://w3c.github.io/webappsec-csp/#directive-style-src-elem
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src-elem
    */
    StyleSrcElem,
    /**
    The `trusted-types` directive instructs the user agent to restrict the
    creation of Trusted Types policies.

    - https://w3c.github.io/trusted-types/dist/spec/#trusted-types-csp-directive
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/trusted-types
    */
    TrustedTypes,
    /**
    The `webrtc` directive restricts whether Connections may be established via
    WebRTC.

    - https://w3c.github.io/webappsec-csp/#directive-webrtc
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/webrtc
    */
    Webrtc,
    /**
    The `worker-src` directive restricts the URLs which may be loaded as a
    `Worker`, `SharedWorker`, or `ServiceWorker`.

    - https://w3c.github.io/webappsec-csp/#directive-worker-src
    - https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/worker-src
    */
    WorkerSrc,
    /**
    Any other CSP directive name not explicitly enumerated above.

    - https://w3c.github.io/webappsec-csp/#csp-directives
    - https://developer.mozilla.org/en-US/docs/Web/API/CSPViolationReport#effectivedirective
    */
    #[serde(untagged)]
    Other(String),
}
