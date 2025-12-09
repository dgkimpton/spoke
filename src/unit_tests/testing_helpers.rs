use proc_macro2::TokenStream;

pub(crate) trait SurroundingString {
    fn surround(input: &str) -> String;
}

pub(crate) trait TokenMatcher {
    fn matches(self, expected: Expected);
    fn matches_inside<T: SurroundingString>(self, expected: Expected);
}

impl TokenMatcher for TokenStream {
    #[track_caller]
    fn matches(self, expected: Expected) {
        matches_stream(self, expected);
    }
    #[track_caller]
    fn matches_inside<T: SurroundingString>(self, expected: Expected) {
       matches_stream(self, Expected(T::surround(expected.0).as_str()));
    }
}

pub(crate) struct Input<'a>(pub(crate) &'a str);
impl<'a> Input<'a> {
    #[track_caller]
    pub(crate) fn stream(&self) -> TokenStream {
        self.0
            .parse::<proc_macro2::TokenStream>()
            .inspect_err(|e| eprintln!("ERROR: {e}"))
            .expect(
                format!(
                    "the input string represents a valid input stream of tokens :: {}",
                    self.0
                )
                .as_str(),
            )
    }
}

pub(crate) struct Expected<'a>(pub(crate) &'a str);
impl<'a> Expected<'a> {
    #[track_caller]
    pub(crate) fn stream(&self) -> proc_macro2::TokenStream {
        self.0
            .parse::<proc_macro2::TokenStream>()
            .inspect_err(|e| eprintln!("ERROR: {e}"))
            .expect(format!("the expected output to be valid rust :: {}", self.0).as_str())
    }
}

#[track_caller]
fn matches_stream(result: proc_macro2::TokenStream, expected: Expected) {
    assert_eq!(expected.stream().to_string(), result.to_string());
}