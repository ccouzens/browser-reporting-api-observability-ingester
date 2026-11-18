use serde::{Deserialize, Serialize};

/**
The string identifying the policy-controlled feature whose policy has been
violated.

- <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-featureid>
- <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#featureid>
- <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy#directives>
- <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/services/network/public/cpp/permissions_policy/permissions_policy_features.json5>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionsPolicyFeature {
    /**
    Controls whether the current document is allowed to gather information
    about the acceleration of the device through the `Accelerometer` interface.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/accelerometer>
    */
    Accelerometer,
    /**
    Controls access to the `getDisplayMediaSet()` API for capturing all
    screens.
    */
    AllScreensCapture,
    /**
    Controls whether the current document is allowed to gather information
    about the amount of light in the environment around the device through the
    `AmbientLightSensor` interface.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/ambient-light-sensor>
    */
    AmbientLightSensor,
    /**
    Controls whether the current document is allowed to use the `ariaNotify()`
    method to fire screen reader announcements.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/aria-notify>
    */
    AriaNotify,
    /**
    Controls whether the current document is allowed to use the Attribution
    Reporting API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/attribution-reporting>
    */
    AttributionReporting,
    /**
    Controls whether the current document is allowed to use autofill.
    */
    Autofill,
    /**
    Controls whether the current document is allowed to autoplay media
    requested through the `HTMLMediaElement` interface.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/autoplay>
    */
    Autoplay,
    /**
    Controls whether the use of the Web Bluetooth API is allowed.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/bluetooth>
    */
    Bluetooth,
    /**
    Controls access to the Topics API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/browsing-topics>
    */
    BrowsingTopics,
    /**
    Controls whether the current document is allowed to use video input
    devices.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/camera>
    */
    Camera,
    /**
    Controls whether or not the document is permitted to use the Captured
    Surface Control API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/captured-surface-control>
    */
    CapturedSurfaceControl,
    /**
    Controls whether the `Sec-CH-Device-Memory` client hint is sent.
    */
    ChDeviceMemory,
    /**
    Controls whether the `Sec-CH-Downlink` client hint is sent.
    */
    ChDownlink,
    /**
    Controls whether the `Sec-CH-DPR` client hint is sent.
    */
    ChDpr,
    /**
    Controls whether the `Sec-CH-ECT` client hint is sent.
    */
    ChEct,
    /**
    Controls whether the `Sec-CH-Prefers-Color-Scheme` client hint is sent.
    */
    ChPrefersColorScheme,
    /**
    Controls whether the `Sec-CH-Prefers-Reduced-Motion` client hint is sent.
    */
    ChPrefersReducedMotion,
    /**
    Controls whether the `Sec-CH-Prefers-Reduced-Transparency` client hint is
    sent.
    */
    ChPrefersReducedTransparency,
    /**
    Controls whether the `Sec-CH-RTT` client hint is sent.
    */
    ChRtt,
    /**
    Controls whether the `Save-Data` client hint is sent.
    */
    ChSaveData,
    /**
    Controls whether the `Sec-CH-UA` client hint is sent.
    */
    ChUa,
    /**
    Controls whether the `Sec-CH-UA-Arch` client hint is sent.
    */
    ChUaArch,
    /**
    Controls whether the `Sec-CH-UA-Bitness` client hint is sent.
    */
    ChUaBitness,
    /**
    Controls whether the `Sec-CH-UA-Form-Factors` client hint is sent.
    */
    ChUaFormFactors,
    /**
    Controls whether the `Sec-CH-UA-Full-Version` client hint is sent.
    */
    ChUaFullVersion,
    /**
    Controls whether the `Sec-CH-UA-Full-Version-List` client hint is sent.
    */
    ChUaFullVersionList,
    /**
    Controls whether or not the document is permitted to use the
    `NavigatorUAData.getHighEntropyValues()` method to retrieve high-entropy
    user-agent data.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/ch-ua-high-entropy-values>
    */
    ChUaHighEntropyValues,
    /**
    Controls whether the `Sec-CH-UA-Mobile` client hint is sent.
    */
    ChUaMobile,
    /**
    Controls whether the `Sec-CH-UA-Model` client hint is sent.
    */
    ChUaModel,
    /**
    Controls whether the `Sec-CH-UA-Platform` client hint is sent.
    */
    ChUaPlatform,
    /**
    Controls whether the `Sec-CH-UA-Platform-Version` client hint is sent.
    */
    ChUaPlatformVersion,
    /**
    Controls whether the `Sec-CH-UA-WoW64` client hint is sent.
    */
    ChUaWow64,
    /**
    Controls whether the `Sec-CH-Viewport-Height` client hint is sent.
    */
    ChViewportHeight,
    /**
    Controls whether the `Sec-CH-Viewport-Width` client hint is sent.
    */
    ChViewportWidth,
    /**
    Controls whether the `Sec-CH-Width` client hint is sent.
    */
    ChWidth,
    /**
    Controls whether the current document is allowed to read from the
    clipboard.
    */
    ClipboardRead,
    /**
    Controls whether the current document is allowed to write to the clipboard.
    */
    ClipboardWrite,
    /**
    Controls access to the Compute Pressure API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/compute-pressure>
    */
    ComputePressure,
    /**
    Controls whether the current document is allowed to use the Controlled
    Frame API.
    */
    ControlledFrame,
    /**
    Controls whether the current document can be treated as cross-origin
    isolated.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/cross-origin-isolated>
    */
    CrossOriginIsolated,
    /**
    Controls the allocation of the top-level origin's `fetchLater()` quota.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/deferred-fetch>
    */
    DeferredFetch,
    /**
    Controls the allocation of the shared cross-origin subframe `fetchLater()`
    quota.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/deferred-fetch-minimal>
    */
    DeferredFetchMinimal,
    /**
    Controls whether the current document is allowed to access device
    attributes.
    */
    DeviceAttributes,
    /**
    Controls whether the current document is allowed to create digital
    credentials.
    */
    DigitalCredentialsCreate,
    /**
    Controls whether the current document is allowed to get digital
    credentials.
    */
    DigitalCredentialsGet,
    /**
    Controls whether the current document is allowed to use the Direct Sockets
    API.
    */
    DirectSockets,
    /**
    Controls whether the current document is allowed to use multicast in the
    Direct Sockets API.
    */
    DirectSocketsMulticast,
    /**
    Controls whether or not the current document is permitted to use the
    `getDisplayMedia()` method to capture screen contents.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/display-capture>
    */
    DisplayCapture,
    /**
    Controls whether the current document is allowed to use the Encrypted Media
    Extensions API (EME).

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/encrypted-media>
    */
    EncryptedMedia,
    /**
    Controls whether tasks in the frame may execute while not rendered.
    */
    ExecutionWhileNotRendered,
    /**
    Controls whether tasks in the frame may execute while out of the viewport.
    */
    ExecutionWhileOutOfViewport,
    /**
    Controls whether the frame may focus without user activation.
    */
    FocusWithoutUserActivation,
    /**
    Test/origin-trial feature in Chromium.
    */
    Frobulate,
    /**
    Controls whether the current document is allowed to use
    `Element.requestFullscreen()`.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/fullscreen>
    */
    Fullscreen,
    /**
    Controls whether the current document is allowed to use the Gamepad API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/gamepad>
    */
    Gamepad,
    /**
    Controls whether the current document is allowed to use the Geolocation
    Interface.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/geolocation>
    */
    Geolocation,
    /**
    Controls whether the current document is allowed to gather information
    about the orientation of the device through the `Gyroscope` interface.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/gyroscope>
    */
    Gyroscope,
    /**
    Controls whether the current document is allowed to use haptic feedback.
    */
    Haptics,
    /**
    Controls whether the current document is allowed to use the WebHID API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/hid>
    */
    Hid,
    /**
    Controls whether the current document is allowed to use the Federated
    Credential Management API (FedCM).

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/identity-credentials-get>
    */
    IdentityCredentialsGet,
    /**
    Controls whether the current document is allowed to use the Idle Detection
    API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/idle-detection>
    */
    IdleDetection,
    /**
    Controls whether the current document is allowed to use the Keyboard Map
    API.
    */
    KeyboardMap,
    /**
    Controls access to the language detection functionality of the Translator
    and Language Detector APIs.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/language-detector>
    */
    LanguageDetector,
    /**
    Controls access to the Prompt API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/language-model>
    */
    LanguageModel,
    /**
    Controls whether the current document is allowed to gather data on the
    user's locally-installed fonts via `Window.queryLocalFonts()`.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/local-fonts>
    */
    LocalFonts,
    /**
    Controls whether the current document is allowed to make network requests
    to local addresses.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/local-network>
    */
    LocalNetwork,
    /**
    Controls whether the current document is allowed to make network requests
    to local and loopback addresses.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/local-network-access>
    */
    LocalNetworkAccess,
    /**
    Controls whether the current document is allowed to make network requests
    to loopback addresses.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/loopback-network>
    */
    LoopbackNetwork,
    /**
    Controls whether the current document is allowed to gather information
    about the orientation of the device through the `Magnetometer` interface.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/magnetometer>
    */
    Magnetometer,
    /**
    Controls whether the current document is allowed to use manual text
    selection/autofill in Chromium.
    */
    ManualText,
    /**
    Controls whether media playback is allowed while the frame is not visible.
    */
    MediaPlaybackWhileNotVisible,
    /**
    Controls whether the current document is allowed to use audio input
    devices.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/microphone>
    */
    Microphone,
    /**
    Controls whether the current document is allowed to use the Web MIDI API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/midi>
    */
    Midi,
    /**
    Controls access to the on-device speech recognition functionality of the
    Web Speech API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/on-device-speech-recognition>
    */
    OnDeviceSpeechRecognition,
    /**
    Controls whether the current document is allowed to use the WebOTP API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/otp-credentials>
    */
    OtpCredentials,
    /**
    Controls whether the current document is allowed to use the Payment Request
    API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/payment>
    */
    Payment,
    /**
    Controls whether the current document is allowed to play a video in
    Picture-in-Picture mode.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/picture-in-picture>
    */
    PictureInPicture,
    /**
    Controls usage of private state token token-request operations.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/private-state-token-issuance>
    */
    PrivateStateTokenIssuance,
    /**
    Controls usage of private state token token-redemption and
    send-redemption-record operations.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/private-state-token-redemption>
    */
    PrivateStateTokenRedemption,
    /**
    Controls whether the current document is allowed to use the Web
    Authentication API to create new asymmetric key credentials.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/publickey-credentials-create>
    */
    PublickeyCredentialsCreate,
    /**
    Controls whether the current document is allowed to use the Web
    Authentication API to retrieve already stored public-key credentials.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/publickey-credentials-get>
    */
    PublickeyCredentialsGet,
    /**
    Controls whether the current document is allowed to provide remote
    `clientDataJSON` in Web Authentication API calls.
    */
    PublickeyCredentialsRemoteClientDataJson,
    /**
    Controls access to the Rewriter API.
    */
    Rewriter,
    /**
    Controls whether the current document is allowed to use the Screen Wake
    Lock API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/screen-wake-lock>
    */
    ScreenWakeLock,
    /**
    Controls access to the Semantic Embedder API in Chromium.
    */
    SemanticEmbedder,
    /**
    Controls whether the current document is allowed to use the Web Serial API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/serial>
    */
    Serial,
    /**
    Controls whether the current document is allowed to use the Web Smart Card
    API.
    */
    SmartCard,
    /**
    Controls whether the current document is allowed to use the Audio Output
    Devices API to list and select speakers.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/speaker-selection>
    */
    SpeakerSelection,
    /**
    Controls whether a document loaded in a third-party context is allowed to
    use the Storage Access API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/storage-access>
    */
    StorageAccess,
    /**
    Controls whether the current document is allowed to use the Sub Apps API.
    */
    SubApps,
    /**
    Controls access to the Summarizer API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/summarizer>
    */
    Summarizer,
    /**
    Controls whether the current document is allowed to make synchronous
    `XMLHttpRequest` calls.
    */
    SyncXhr,
    /**
    Controls access to the WebMCP / tools API in Chromium.
    */
    Tools,
    /**
    Controls access to the translation functionality of the Translator and
    Language Detector APIs.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/translator>
    */
    Translator,
    /**
    Controls whether the current document is allowed to use unbounded elements.
    */
    UnboundedElement,
    /**
    Controls whether the current document is allowed to run `unload` event
    handlers.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/unload>
    */
    Unload,
    /**
    Controls whether the current document is allowed to use the WebUSB API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/usb>
    */
    Usb,
    /**
    Controls whether the current document is allowed to access unrestricted
    USB devices.
    */
    UsbUnrestricted,
    /**
    Controls whether the current document is allowed to scroll vertically.
    */
    VerticalScroll,
    /**
    Controls whether the current document is allowed to use the Web App
    Installation API.
    */
    WebAppInstallation,
    /**
    Controls whether the current document is allowed to use the Web Printing
    API.
    */
    WebPrinting,
    /**
    Controls whether or not the current document is allowed to use
    `Navigator.share()` of the Web Share API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/web-share>
    */
    WebShare,
    /**
    Controls whether the current document is allowed to use the WebNN API.
    */
    Webnn,
    /**
    Controls whether or not the current document is allowed to use the Window
    Management API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/window-management>
    */
    WindowManagement,
    /**
    Controls access to the Writer API.
    */
    Writer,
    /**
    Controls whether or not the current document is allowed to use the WebXR
    Device API.

    - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy/xr-spatial-tracking>
    */
    XrSpatialTracking,
    /**
    Any other permissions policy feature ID not explicitly enumerated above.

    - <https://w3c.github.io/webappsec-permissions-policy/#dom-permissionspolicyviolationreportbody-featureid>
    - <https://developer.mozilla.org/en-US/docs/Web/API/PermissionsPolicyViolationReport#featureid>
    */
    #[serde(untagged)]
    Other(String),
}
