use crate::{code_block::CodeBlock, name::*, parse, parser::*, token_helpers::*};

pub(crate) struct Assert {
    parent: parse::AnchorParent,
    name: Name,
    left_code: CodeBlock,
}
impl Assert {
    pub(crate) fn from_suite(parent: parse::Suite, name: Name) -> Self {
        Self::new(parse::AnchorParent::from_suite(parent), name)
    }

    pub(crate) fn new(parent: parse::AnchorParent, name: Name) -> Self {
        Self {
            parent,
            name,
            left_code: CodeBlock::new(),
        }
    }

    fn generate_assert_test_into(&mut self, target: &mut SuiteGenerator) {
        let mut test = self.parent.populate_test(TestCase::new(
            self.parent
                .collect_name_parts(CompoundName::new())
                .followed_by(&self.name)
                .function_name(),
        ));

        let location = self.name.span();

        test.push_code([
            ident("assert", location),
            punct('!', location),
            parenthesised(std::mem::take(&mut self.left_code).into_iter(), location),
            punct(';', location),
        ]);

        target.push_test(test);
    }
}

impl Parser for Assert {
    fn accept_token(mut self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == '$' => {
                parse::TransientAssertAnchor::new(self.parent, self.name, self.left_code, &punct)
                     .consumed_token()
            }

            TokenTree::Punct(punct) if punct.as_char() == ';' => {
                if self.left_code.is_empty() {
                    target.push_new_error(
                        &self.name,
                        "expected an assertion or test body after the name, but found `;`",
                    );
                } else {
                    self.generate_assert_test_into(target);
                }
                self.parent.continuation()
            }

            other => {
                self.left_code.push(other);
                self.consumed_token()
            }
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &self.name.span(),
            "reached end of group input before finding details of the named assertion. Missing ; ?",
        );

        self.parent.continuation()
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.name.span(),
            "reached end of input before finding details of the named assertion. Missing ; ?",
        );
    }
}
