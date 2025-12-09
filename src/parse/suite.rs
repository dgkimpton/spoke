use crate::{parse, parser::*};

pub(crate) struct Suite();
impl Parser for Suite {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == '$' => {
                parse::TransientSuiteAnchor::new(self, &punct).consumed_token()
            }
            preamble => {
                target.push_preamble(preamble);
                self.consumed_token()
            }
        }
    }

    fn end_of_stream(self, _: &mut SuiteGenerator) {}

    fn end_of_group(self, _: &mut SuiteGenerator) -> ParseRule {
        panic!("this state shouldn't be possible")
    }
}
