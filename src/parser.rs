pub(crate) use crate::{generator::*, parse_rule::*, span_source::*};
pub(crate) use proc_macro2::{Delimiter, TokenTree};

pub(crate) trait Parser {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule;
    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule;
    fn end_of_stream(self, target: &mut SuiteGenerator);
}