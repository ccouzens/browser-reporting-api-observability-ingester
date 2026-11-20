use serde::{Deserialize, Serialize};

/**
The visibility state of a document, matching the HTML `DocumentVisibilityState`
enum (`"visible"` or `"hidden"`).

- <https://html.spec.whatwg.org/multipage/interaction.html#documentvisibilitystate>
- <https://wicg.github.io/crash-reporting/#crashreportbody-visibility_state>
- <https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState>
- <https://developer.mozilla.org/en-US/docs/Web/API/CrashReport#visibility_state>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentVisibilityState {
    /**
    The document content is at least partially visible.

    - <https://html.spec.whatwg.org/multipage/interaction.html#dom-documentvisibilitystate-visible>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CrashReport#visibility_state>
    */
    Visible,
    /**
    The document content is completely hidden.

    - <https://html.spec.whatwg.org/multipage/interaction.html#dom-documentvisibilitystate-hidden>
    - <https://developer.mozilla.org/en-US/docs/Web/API/CrashReport#visibility_state>
    */
    Hidden,
}
