/**
A single zero-allocation token produced from a `User-Agent` string.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UaToken<'a> {
    NameSepNumber { name: &'a str, number: &'a str },
    RoundBrackets(&'a str),
    SquareBrackets(&'a str),
    Other(&'a str),
}

pub(super) struct Tokenizer<'a> {
    rest: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub(super) const fn new(input: &'a str) -> Self {
        Self { rest: input }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = UaToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rest = self.rest.trim_start();
        if self.rest.is_empty() {
            return None;
        }

        if let Some(after_open) = self.rest.strip_prefix('(') {
            let (content, remaining) = split_round_brackets(after_open);
            self.rest = remaining;
            return Some(UaToken::RoundBrackets(content));
        }

        if let Some(after_open) = self.rest.strip_prefix('[') {
            let (content, remaining) = after_open.split_once(']').unwrap_or((after_open, ""));
            self.rest = remaining;
            return Some(UaToken::SquareBrackets(content));
        }

        let name_end = self
            .rest
            .find(['/', ' ', '\t', '(', '['])
            .unwrap_or(self.rest.len());
        let (name, after_name) = self.rest.split_at(name_end);

        if !name.is_empty()
            && let Some((sep, after_sep)) =
                after_name.strip_prefix('/').map(|s| ('/', s)).or_else(|| {
                    after_name
                        .strip_prefix([' ', '\t'])
                        .map(|s| (' ', s.trim_start()))
                })
            && !after_sep.starts_with(['(', '['])
        {
            let (number, next_rest) = split_word(after_sep);
            if number.as_bytes().first().is_some_and(u8::is_ascii_digit)
                && (sep == '/' || !number.contains('/'))
            {
                self.rest = next_rest;
                return Some(UaToken::NameSepNumber { name, number });
            }
        }

        let (word, rest) = split_word(self.rest);
        self.rest = rest;
        Some(UaToken::Other(word))
    }
}

fn split_round_brackets(after_open: &str) -> (&str, &str) {
    let mut depth = 1usize;
    for (idx, b) in after_open.bytes().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return (&after_open[..idx], &after_open[idx + 1..]);
                }
            }
            _ => {}
        }
    }
    (after_open, "")
}

fn split_word(s: &str) -> (&str, &str) {
    let end = s.find([' ', '\t', '(', '[']).unwrap_or(s.len());
    s.split_at(end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_chrome_mobile() {
        let ua = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Mobile Safari/537.36";
        let tokens: Vec<_> = Tokenizer::new(ua).collect();
        assert_eq!(
            tokens,
            vec![
                UaToken::NameSepNumber {
                    name: "Mozilla",
                    number: "5.0",
                },
                UaToken::RoundBrackets("Linux; Android 10; K"),
                UaToken::NameSepNumber {
                    name: "AppleWebKit",
                    number: "537.36",
                },
                UaToken::RoundBrackets("KHTML, like Gecko"),
                UaToken::NameSepNumber {
                    name: "Chrome",
                    number: "157.0.0.0",
                },
                UaToken::Other("Mobile"),
                UaToken::NameSepNumber {
                    name: "Safari",
                    number: "537.36",
                },
            ]
        );
    }

    #[test]
    fn test_tokenize_facebook_with_spaces_in_brackets() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/22A3354 [FBAN/Messenger for iOS;FBAV/514.0.0.47.102;FBCR/Life Wireless;]";
        let tokens: Vec<_> = Tokenizer::new(ua).collect();
        assert_eq!(
            tokens,
            vec![
                UaToken::NameSepNumber {
                    name: "Mozilla",
                    number: "5.0",
                },
                UaToken::RoundBrackets("iPhone; CPU iPhone OS 18_0 like Mac OS X"),
                UaToken::NameSepNumber {
                    name: "AppleWebKit",
                    number: "605.1.15",
                },
                UaToken::RoundBrackets("KHTML, like Gecko"),
                UaToken::NameSepNumber {
                    name: "Mobile",
                    number: "22A3354",
                },
                UaToken::SquareBrackets(
                    "FBAN/Messenger for iOS;FBAV/514.0.0.47.102;FBCR/Life Wireless;"
                ),
            ]
        );
    }

    #[test]
    fn test_tokenize_instagram_space_number() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Instagram 360.0.0.0 (iPhone16,1; iOS 18_0)";
        let tokens: Vec<_> = Tokenizer::new(ua).collect();
        assert_eq!(
            tokens,
            vec![
                UaToken::NameSepNumber {
                    name: "Mozilla",
                    number: "5.0",
                },
                UaToken::RoundBrackets("iPhone; CPU iPhone OS 18_0 like Mac OS X"),
                UaToken::NameSepNumber {
                    name: "AppleWebKit",
                    number: "605.1.15",
                },
                UaToken::RoundBrackets("KHTML, like Gecko"),
                UaToken::NameSepNumber {
                    name: "Mobile",
                    number: "15E148",
                },
                UaToken::NameSepNumber {
                    name: "Instagram",
                    number: "360.0.0.0",
                },
                UaToken::RoundBrackets("iPhone16,1; iOS 18_0"),
            ]
        );
    }

    #[test]
    fn test_tokenize_nested_round_brackets() {
        let ua = "Mozilla/5.0 (Linux; Android 11; moto g(7) power Build/QPO30.52-29; wv) AppleWebKit/537.36";
        let tokens: Vec<_> = Tokenizer::new(ua).collect();
        assert_eq!(
            tokens,
            vec![
                UaToken::NameSepNumber {
                    name: "Mozilla",
                    number: "5.0",
                },
                UaToken::RoundBrackets("Linux; Android 11; moto g(7) power Build/QPO30.52-29; wv"),
                UaToken::NameSepNumber {
                    name: "AppleWebKit",
                    number: "537.36",
                },
            ]
        );
    }
}
