use crate::{error::*, token_helpers::*};

pub(crate) trait TokenMatcher {
    fn matches_ok(self, expected: Expected);
    fn matches_failure(self, expected: Expected);
}

impl TokenMatcher for CompileResult<Vec<proc_macro2::TokenTree>> {
    fn matches_ok(self, expected: Expected) {
        matches_tokens(self.ok().expect("parsed ok"), expected)
    }

    fn matches_failure(self, expected: Expected) {
        matches_tokens(
            self.err()
                .expect("parsed with compile error")
                .generate_tokens(),
            expected,
        )
    }
}
impl TokenMatcher for CompileResult<proc_macro2::TokenStream> {
    fn matches_ok(self, expected: Expected) {
        matches_stream(self.ok().expect("parsed ok"), expected)
    }

    fn matches_failure(self, expected: Expected) {
        matches_tokens(
            self.err()
                .expect("parsed with compile error")
                .generate_tokens(),
            expected,
        )
    }
}

pub(crate) struct Input<'a>(pub (crate) &'a str);
impl<'a> Input<'a> {
    pub(crate) fn stream(&self) -> CompileResult<proc_macro2::TokenStream> {
        Ok(self
            .0
            .parse::<proc_macro2::TokenStream>()
            .inspect_err(|e| eprintln!("ERROR: {e}"))
            .expect("the input string represents a valid input stream of tokens"))
    }
}

pub(crate) struct Expected<'a>(pub (crate) &'a str);
impl<'a> Expected<'a> {
    pub(crate) fn stream(&self) -> proc_macro2::TokenStream {
        self.0
            .parse::<proc_macro2::TokenStream>()
            .inspect_err(|e| eprintln!("ERROR: {e}"))
            .expect("the expected output to be valid rust")
    }
}

fn matches_stream(result: proc_macro2::TokenStream, expected: Expected) {
    assert_eq!(expected.stream().to_string(), result.to_string());
}

fn matches_tokens(result: impl IterableTokens, expected: Expected) {
    assert_eq!(expected.stream().to_string(), in_stream(result).to_string());
}
