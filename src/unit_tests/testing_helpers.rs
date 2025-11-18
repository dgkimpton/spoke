use proc_macro2::TokenStream;

use crate::{ generator::{IntoVecTokens, TokenGenerator}, token_helpers::*};

pub(crate) struct TestResult(Vec<proc_macro2::TokenTree>);

pub(crate) trait GenerateTestTokens {
    fn generate(&mut self) -> TestResult;
}

impl<T: TokenGenerator> GenerateTestTokens for T {
    fn generate(&mut self) -> TestResult {
        TestResult(self.into_vec())
    }
}

pub(crate) trait TokenMatcher {
    fn matches(self, expected: Expected);
}

impl TokenMatcher for TestResult {
    fn matches(self, expected: Expected) {
        matches_tokens(self.0, expected)
    }
}

impl TokenMatcher for TokenStream {
    fn matches(self, expected: Expected) {
        matches_stream(self, expected);
    }
}

pub(crate) struct Input<'a>(pub (crate) &'a str);
impl<'a> Input<'a> {
    pub(crate) fn stream(&self) -> TokenStream {
        self
            .0
            .parse::<proc_macro2::TokenStream>()
            .inspect_err(|e| eprintln!("ERROR: {e}"))
            .expect("the input string represents a valid input stream of tokens")
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
