pub(crate) use crate::{generator::*, parse_rule::*, span_source::*};
pub(crate) use proc_macro2::{Delimiter, TokenTree};

use crate::token_helpers::*;

pub(crate) trait Parser {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule;
    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule;
    fn end_of_stream(self, target: &mut SuiteGenerator);
}

pub(crate) trait TokenStreamExt {
    fn process_into(self, consumer: ParseRule, target: &mut SuiteGenerator);
}

impl TokenStreamExt for TokenStream {
    fn process_into(self, mut current_rule: ParseRule, target: &mut SuiteGenerator) {
        for token in self.into_iter() {
            current_rule = current_rule.accept_token(token, target);
        }

        current_rule.end_of_stream(target);
    }
}

pub(crate) trait GroupExt {
    fn process_into(self, consumer: ParseRule, target: &mut SuiteGenerator) -> ParseRule;
}

impl GroupExt for Group {
    fn process_into(self, mut current_rule: ParseRule, target: &mut SuiteGenerator) -> ParseRule {
        for token in self.stream().into_iter() {
            current_rule = current_rule.accept_token(token, target);
        }

        current_rule.end_of_group(target)
    }
}
