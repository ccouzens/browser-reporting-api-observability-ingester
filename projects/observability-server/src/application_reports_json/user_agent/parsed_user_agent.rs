use super::{
    Browser, Engine, ReportedOs,
    tokenizer::{Tokenizer, UaToken},
};
use serde::Serialize;

/**
Structured classification parsed lazily from a [`super::UserAgent`] string.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ParsedUserAgent<'a> {
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_mobile: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_webview: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_bot: bool,
    pub reported_os: ReportedOs,
    pub engine: Engine,
    pub browser: Option<Browser<'a>>,
}

impl<'a> ParsedUserAgent<'a> {
    pub fn parse(ua: &'a str) -> Self {
        let mut reported_os = ReportedOs::Other;
        let mut is_mobile = false;
        let mut is_bot = false;
        let mut saw_wv = false;

        let mut firefox = None;
        let mut chrome = None;
        let mut version = None;
        let mut has_safari = false;
        let mut has_apple_webkit = false;
        let mut specific_browser = None;
        let mut bot_browser = None;

        let mut tokens = Tokenizer::new(ua);
        while let Some(token) = tokens.next() {
            match token {
                UaToken::RoundBrackets(comment) => {
                    let mut segments = comment.split(';').map(str::trim);
                    let mut first_segment = None;
                    while let Some(segment) = segments.next() {
                        if first_segment.is_none() {
                            first_segment = Some(segment);
                        }
                        match segment {
                            "Mobile" => is_mobile = true,
                            "wv" => saw_wv = true,
                            "compatible" => {
                                is_bot = true;
                                if let Some(next) = segments.next()
                                    && let Some(browser) = parse_comment_browser(next)
                                {
                                    bot_browser = Some(browser);
                                }
                            }
                            _ if segment.starts_with("+http://")
                                || segment.starts_with("+https://") =>
                            {
                                is_bot = true;
                                if bot_browser.is_none()
                                    && let Some(first) = first_segment
                                    && !first.starts_with("+http://")
                                    && !first.starts_with("+https://")
                                    && !first.starts_with("http://")
                                    && !first.starts_with("https://")
                                {
                                    bot_browser = parse_comment_browser(first);
                                }
                            }
                            _ if let Some(via) = segment.strip_prefix("via ") => {
                                is_bot = true;
                                if let Some(name) = via.split_whitespace().last() {
                                    bot_browser = Some(Browser {
                                        name,
                                        version: None,
                                    });
                                }
                            }
                            _ if let Some(browser) = parse_comment_at_browser(segment) => {
                                specific_browser = Some(browser);
                            }
                            _ => {
                                for word in segment.split_whitespace() {
                                    if let Some(os) = ReportedOs::from_token(word)
                                        && matches!(
                                            reported_os,
                                            ReportedOs::Other | ReportedOs::Linux
                                        )
                                    {
                                        reported_os = os;
                                    }
                                }
                            }
                        }
                    }
                }
                UaToken::NameSepNumber { name, number } => match name {
                    "Mozilla" | "Gecko" | "FBNV" | "NW" | "NV" => {}
                    "AppleWebKit" => has_apple_webkit = true,
                    "Mobile" => is_mobile = true,
                    "Version" => version = parse_version(number),
                    "Safari" => has_safari = true,
                    "Chrome" => chrome = parse_version(number),
                    "HeadlessChrome" => {
                        let parsed = parse_version(number);
                        is_bot = true;
                        chrome = parsed;
                        specific_browser = Some(Browser {
                            name,
                            version: parsed,
                        });
                    }
                    "Firefox" => firefox = parse_version(number),
                    _ => {
                        if is_bot_name(name) {
                            is_bot = true;
                        }
                        specific_browser = Some(Browser {
                            name,
                            version: parse_version(number),
                        });
                    }
                },
                UaToken::SquareBrackets(content) => {
                    let mut is_facebook = false;
                    let mut fb_version = None;
                    for entry in content.split(';') {
                        if let Some((key, val)) = entry.split_once('/') {
                            match key {
                                "FBAN" | "FB_IAB" => is_facebook = true,
                                "FBAV" => {
                                    is_facebook = true;
                                    fb_version = parse_version(val);
                                }
                                _ => {}
                            }
                        }
                    }
                    if is_facebook {
                        saw_wv = true;
                        specific_browser = Some(Browser {
                            name: "Facebook",
                            version: fb_version,
                        });
                    } else if !content.contains(';')
                        && let Some((name, val)) = content.split_once('/')
                        && !name.is_empty()
                        && let Some(ver) = parse_version(val)
                    {
                        if is_bot_name(name) {
                            is_bot = true;
                        }
                        specific_browser = Some(Browser {
                            name,
                            version: Some(ver),
                        });
                    }
                }
                UaToken::Other("Mobile") => is_mobile = true,
                UaToken::Other(
                    "Mozilla" | "Gecko" | "AppleWebKit" | "Version" | "Safari" | "Chrome"
                    | "Firefox" | "like",
                ) => {}
                UaToken::Other("compatible") => {
                    is_bot = true;
                    match tokens.next() {
                        Some(UaToken::NameSepNumber { name, number }) => {
                            bot_browser = Some(Browser {
                                name,
                                version: parse_version(number),
                            });
                        }
                        Some(UaToken::Other(name)) => {
                            bot_browser = Some(Browser {
                                name,
                                version: None,
                            });
                        }
                        _ => {}
                    }
                }
                UaToken::Other(word) => {
                    if specific_browser.is_none()
                        && word
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
                    {
                        if is_bot_name(word) {
                            is_bot = true;
                        }
                        specific_browser = Some(Browser {
                            name: word,
                            version: None,
                        });
                    }
                }
            }
        }

        let browser = bot_browser
            .or(specific_browser)
            .or_else(|| {
                firefox.map(|v| Browser {
                    name: "Firefox",
                    version: Some(v),
                })
            })
            .or_else(|| {
                chrome.map(|v| Browser {
                    name: "Chrome",
                    version: Some(v),
                })
            })
            .or_else(|| {
                (has_safari && version.is_some()).then_some(Browser {
                    name: "Safari",
                    version,
                })
            })
            .or_else(|| {
                (has_apple_webkit && matches!(reported_os, ReportedOs::Ios | ReportedOs::MacOs))
                    .then_some(Browser {
                        name: "WKWebView",
                        version: None,
                    })
            });

        let engine = if let Some((major, _)) = chrome {
            Engine::Blink { major }
        } else if let Some((major, _)) = firefox {
            Engine::Gecko { major }
        } else if has_apple_webkit {
            Engine::WebKit { version }
        } else {
            Engine::Other
        };

        let is_webview = !is_bot
            && ((reported_os == ReportedOs::Android
                && (saw_wv
                    || (matches!(engine, Engine::Blink { .. }) && version == Some((4, 0)))))
                || (matches!(engine, Engine::WebKit { .. })
                    && matches!(reported_os, ReportedOs::Ios | ReportedOs::MacOs)
                    && !browser.is_some_and(|b| b.name == "Safari")));

        Self {
            is_mobile,
            is_webview,
            is_bot,
            reported_os,
            engine,
            browser,
        }
    }
}

fn parse_comment_browser(segment: &str) -> Option<Browser<'_>> {
    let token = segment.split_whitespace().next()?;
    let (name, version) = token
        .split_once('/')
        .map_or((token, None), |(n, v)| (n, parse_version(v)));
    Some(Browser { name, version })
}

fn parse_comment_at_browser(segment: &str) -> Option<Browser<'_>> {
    let (name, rest) = segment.split_once(' ')?;
    let (tag, version_str) = rest.split_once('@')?;
    if name.is_empty() || tag.is_empty() || version_str.contains(char::is_whitespace) {
        return None;
    }
    let version = parse_version(version_str)?;
    Some(Browser {
        name,
        version: Some(version),
    })
}

fn is_bot_name(name: &str) -> bool {
    name == "Chrome-Lighthouse"
        || name.starts_with("meta-")
        || [
            "bot", "Bot", "spider", "Spider", "crawler", "Crawler", "proxy", "Proxy",
        ]
        .iter()
        .any(|suffix| name.ends_with(suffix))
}

fn parse_version(version: &str) -> Option<(u16, u16)> {
    let version = version.strip_prefix(['v', 'V']).unwrap_or(version);
    let mut parts = version.split('.');
    let major = parse_leading_u16(parts.next()?)?;
    let minor = parts.next().and_then(parse_leading_u16).unwrap_or(0);
    Some((major, minor))
}

fn parse_leading_u16(s: &str) -> Option<u16> {
    let digits_end = s
        .bytes()
        .position(|b| !b.is_ascii_digit())
        .unwrap_or(s.len());
    s[..digits_end].parse::<u16>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application_reports_json::UserAgent;
    use assert_json_diff::assert_json_eq;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TestCase {
        user_agent: String,
        #[serde(flatten)]
        expected: serde_json::Value,
    }

    #[test]
    fn test_parse_user_agent_cases() {
        let cases: Vec<TestCase> =
            serde_json::from_str(include_str!("parsed_user_agent_test_cases.json")).unwrap();
        assert!(!cases.is_empty());

        for case in cases {
            let actual =
                serde_json::to_value(UserAgent::from(case.user_agent.as_str()).parse()).unwrap();
            assert_json_eq!(actual, case.expected);
        }
    }

    #[test]
    fn engine_and_os_helpers() {
        assert_eq!(Engine::Blink { major: 156 }.as_str(), "blink");
        assert_eq!(Engine::Blink { major: 156 }.version(), Some((156, 0)));
        assert_eq!(Engine::Gecko { major: 160 }.as_str(), "gecko");
        assert_eq!(
            Engine::WebKit {
                version: Some((27, 2))
            }
            .as_str(),
            "webkit"
        );
        assert_eq!(
            Engine::WebKit {
                version: Some((27, 2))
            }
            .version(),
            Some((27, 2))
        );
        assert_eq!(Engine::Other.as_str(), "other");
        assert_eq!(Engine::Other.version(), None);
        assert_eq!(ReportedOs::Linux.as_str(), "linux");
    }
}
