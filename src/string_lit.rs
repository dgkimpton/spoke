use std::collections::VecDeque;

use proc_macro2::Literal;

pub(crate) trait TokenExtensions {
    fn as_string_literal(&self) -> Result<String, String>;
}

impl TokenExtensions for Literal {
    fn as_string_literal(&self) -> Result<String, String> {
        string_as_string_literal(self.to_string())
    }
}

fn string_as_string_literal(input: impl AsRef<str>) -> Result<String, String> {
    let mut chars = input.as_ref().chars().collect::<VecDeque<char>>();

    let full_len = chars.len();
    let mut is_raw = false;

    loop {
        if chars.len() < 2 {
            return Err(format!("string too short"));
        }

        match chars.pop_front() {
            None => return Err(format!("no first character")),

            Some('r') => {
                if chars.len() + 1 == full_len {
                    is_raw = true;
                } else {
                    return Err(format!("found r at a position other than the start"));
                }
            }

            Some('"') => match chars.pop_back() {
                Some('"') => {
                    if is_raw {
                        return Ok(chars.iter().collect());
                    } else {
                        return verify_quoted(chars);
                    }
                }
                Some(c) => {
                    let mut suffix = String::new();
                    suffix.push(c);
                    while let Some(back) = chars.pop_back() {
                        match back {
                            '"' => {
                                return Err(format!(
                                    "unmatched suffix detected, `{}`, did you miss a space?",
                                    suffix
                                ));
                            }
                            c => suffix.insert(0, c),
                        }
                    }
                    return Err(format!("missing closing quote on string"));
                }
                None => return Err(format!("unbalanced surrounding quotes")),
            },

            Some('#') if is_raw => match chars.pop_back() {
                Some('#') => {
                    continue;
                }
                Some(c) => {
                    let mut suffix = String::new();
                    suffix.push(c);
                    while let Some(back) = chars.pop_back() {
                        match back {
                            '#' => {
                                if let Some('"') = chars.pop_back() {
                                    return Err(format!(
                                        "unmatched raw suffix detected, `{}`, did you miss a space?",
                                        suffix
                                    ));
                                } else {
                                    return Err(format!("bad raw string format"));
                                }
                            }
                            c => suffix.insert(0, c),
                        }
                    }
                    return Err(format!("missing closing hash on raw string"));
                }
                None => {
                    return Err(format!("unbalanced surrounding hashes"));
                }
            },

            Some(c) => return Err(format!("unexpected character {c}")),
        }
    }
}

fn verify_quoted(chars: VecDeque<char>) -> Result<String, String> {
    let mut result = String::new();
    let mut previous: Option<char> = None;
    for c in chars {
        match c {
            '"' => {
                if previous != Some('\\') {
                    return Err("literal contains unescaped quote character(s)".to_string());
                }
            }
            o => previous = Some(o),
        }
        result.push(c);
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_literal_is_error() {
        assert_eq!(super::string_as_string_literal(""), err("string too short"));
    }

    #[test]
    fn literal_without_quotes_is_error() {
        assert_eq!(
            super::string_as_string_literal("asss"),
            err("unexpected character a")
        );
    }

    #[test]
    fn single_double_quote_is_error() {
        assert_eq!(
            super::string_as_string_literal(r#"""#),
            err("string too short")
        );
    }

    #[test]
    fn paired_double_quote_is_empty_string() {
        assert_eq!(super::string_as_string_literal(r#""""#), ok(""))
    }

    #[test]
    fn quoted_character_is_string() {
        assert_eq!(super::string_as_string_literal(r#""a""#), ok("a"));
    }

    #[test]
    fn quoted_string_is_string() {
        assert_eq!(
            super::string_as_string_literal(r#""some string""#),
            ok("some string")
        );
    }

    #[test]
    fn quoted_string_with_internal_quote_is_error() {
        assert_eq!(
            super::string_as_string_literal(r#""some " string""#),
            err("literal contains unescaped quote character(s)")
        );
    }

    #[test]
    fn single_r_raw_string_empty_ok() {
        assert_eq!(super::string_as_string_literal(r#"r"""#), ok(""));
    }

    #[test]
    fn single_r_raw_string_char_ok() {
        assert_eq!(super::string_as_string_literal(r#"r"a""#), ok("a"));
    }

    #[test]
    fn raw_string_with_balanced_hashes_is_ok() {
        assert_eq!(
            super::string_as_string_literal(r#####"r####"a"####"#####),
            ok("a")
        );
    }

    #[test]
    fn raw_string_with_quotes_is_ok() {
        assert_eq!(
            super::string_as_string_literal(r#####"r####"a"b"####"#####),
            ok("a\"b")
        );
    }

    #[test]
    fn raw_string_with_hashes_is_ok() {
        assert_eq!(
            super::string_as_string_literal(r#####"r####"a"#b"####"#####),
            ok("a\"#b")
        );
    }

    #[test]
    fn string_with_suffix_is_error() {
        assert_eq!(
            super::string_as_string_literal(r#""str"suffix"#),
            err("unmatched suffix detected, `suffix`, did you miss a space?")
        );
    }

    #[test]
    fn r_at_position_other_than_start() {
        assert_eq!(
            super::string_as_string_literal(r###"r#r""##"###),
            err("found r at a position other than the start")
        );
    }
    #[test]
    fn unbalanced_hashes() {
        assert_eq!(
            super::string_as_string_literal(r###"r##""#"###),
            err("missing closing hash on raw string")
        );
    }
    #[test]
    fn unmatched_suffix_with_hashes() {
        assert_eq!(
            super::string_as_string_literal(r###"r##""r##"###),
            err("unmatched suffix detected, `r`, did you miss a space?")
        );
    }

    #[test]
    fn missing_closing_hash() {
        assert_eq!(
            super::string_as_string_literal(r###"r##""#"###),
            err("missing closing hash on raw string")
        );
    }
    #[test]
    fn unmatched_raw_suffix() {
        assert_eq!(
            super::string_as_string_literal(r###"r##""#c"###),
            err("unmatched raw suffix detected, `c`, did you miss a space?")
        );
    }
    #[test]
    fn bad_raw_string_format() {
        assert_eq!(
            super::string_as_string_literal(r###"r##"" #c"###),
            err("bad raw string format")
        );
    }

    fn err<T>(error: impl Into<String>) -> Result<T, String> {
        Err(error.into())
    }

    fn ok<T>(error: impl Into<String>) -> Result<String, T> {
        Ok(error.into())
    }
}
