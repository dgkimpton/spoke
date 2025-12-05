use crate::parser::*;

pub(crate) struct Finished();

impl Parser for Finished {
    fn accept_token(self, _: TokenTree, _: &mut SuiteGenerator) -> ParseRule {
        // once we are finished just drop any remaining tokens in the input
        self.consumed_token()
    }

    fn end_of_stream(self, _: &mut SuiteGenerator) {}

    fn end_of_group(self, _: &mut SuiteGenerator) -> ParseRule {
        ParseRule::Finished(self)
    }
}
