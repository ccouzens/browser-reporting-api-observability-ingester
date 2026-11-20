use serde::{Deserialize, Serialize};

/**
A more specific classification of the type of crash that occured, if known.

- <https://wicg.github.io/crash-reporting/#dom-crashreportbody-reason>
- <https://wicg.github.io/crash-reporting/#crashreportbody-reason>
- <https://developer.mozilla.org/en-US/docs/Web/API/CrashReport#reason>
- <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/content/browser/renderer_host/render_frame_host_impl.cc>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CrashReason {
    /**
    The page ran out of memory.

    - <https://wicg.github.io/crash-reporting/#crashreportbody-reason>
    - <https://wicg.github.io/crash-reporting/#concept-oom>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CrashReport#reason>
    */
    Oom,
    /**
    The page was killed due to being unresponsive.

    - <https://wicg.github.io/crash-reporting/#crashreportbody-reason>
    - <https://wicg.github.io/crash-reporting/#concept-unresponsive>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CrashReport#reason>
    */
    Unresponsive,
    /**
    Any other crash reason string not explicitly enumerated above.
    */
    #[serde(untagged)]
    Other(String),
}
