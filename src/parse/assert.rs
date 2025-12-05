use crate::{code_block::CodeBlock, name::*, parse, parser::*};

pub(crate) struct Assert {
    parent: parse::Body,
    name: Name,
    left_code: CodeBlock,
}
impl Assert {
    pub(crate) fn new(parent: parse::Body, name: Name) -> Self {
        Self {
            parent,
            name,
            left_code: CodeBlock::new(),
        }
    }
}

impl Parser for Assert {
    fn accept_token(mut self, token: TokenTree, _: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == '$' => {
                parse::TransientAssertAnchor::new(self.parent, self.name, self.left_code, &punct)
                    .consumed_token()
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
            "RULE::ASSERT: reached end of group input before finding deails of the named assertion",
        );
        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.name.span(),
            "RULE::ASSERT: reached end of input before finding deails of the named assertion",
        );
    }
}
