use std::mem::take;

use crate::{
    code_block::*,
    name::*,
    parse,
    parser::*,
    token_helpers::{ident, parenthesised, punct},
};

pub(crate) struct AssertEq {
    parent: parse::Body,
    name: Name,
    anchor: Span,
    left_code: CodeBlock,
    right_code: CodeBlock,
}

impl AssertEq {
    pub(crate) fn new(
        parent: parse::Body,
        name: Name,
        anchor: &impl SpanSource,
        left_code: CodeBlock,
        location: &impl SpanSource,
    ) -> Self {
        Self {
            parent,
            name,
            anchor: location.span(),
            left_code,
            right_code: CodeBlock::new(),
        }
    }

    fn generate_test(&mut self, target: &mut SuiteGenerator) {
        let mut is_ok = true;

        let left_code = take(&mut self.left_code);

        if left_code.is_empty() {
            target.push_new_error(
                &self.anchor,
                "RULE::AssertEq: no code found for the left side of the equality assertion",
            );
            is_ok = false;
        }

        if self.right_code.is_empty() {
            target.push_new_error(
                &self.anchor,
                "RULE::AssertEq: no code found for the right hand side of the equality assertion",
            );
            is_ok = false;
        }

        if is_ok {
            let mut test = self.parent.populate_test(TestCase::new(
                self.parent
                    .collect_name_parts(CompoundName::new())
                    .followed_by(&self.name)
                    .function_name(),
            ));

            test.push_code([
                ident("assert_eq", self.anchor),
                punct('!', self.anchor),
                parenthesised(
                    left_code
                        .into_iter()
                        .chain([punct(',', self.anchor)])
                        .chain(take(&mut self.right_code).into_iter()),
                    self.anchor,
                ),
                punct(';', self.anchor),
            ]);

            target.push_test(test);
        }
    }
}

impl Parser for AssertEq {
    fn accept_token(mut self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == ';' => {
                self.generate_test(target);
                self.parent.consumed_token()
            }

            other => {
                self.right_code.push(other);
                self.consumed_token()
            }
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &Span::call_site(),
            "RULE::AssertEq: reached end of group input before reaching the end of the equality assertion definition",
        );

        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &Span::call_site(),
            "RULE::AssertEq: reached end of input before reaching the end of the equality assertion definition",
        );
    }
}
