use proc_macro2::{Group, TokenTree};

use crate::{
    asserts::Assert, body::Body, code_block::CodeBlock, consumer::TokenConsumer,
    error::ProcessingError, generator::TokenGenerator, name::Name, token::Token, token_is::TokenIs,
};

pub(crate) enum Content {
    None,
    Body(Body),
    Assert(Assert),
}

impl Content {
    pub(crate) fn body(
        input: Group,
        name: Name,
        code: CodeBlock,
    ) -> (Self, Option<ProcessingError>) {
        let (body, error) = Body::new_from(input.stream(), name, code);
        (Content::Body(body), error)
    }

    pub(crate) fn assert(input: TokenTree, name: Name, code: CodeBlock) -> Self {
        Self::Assert(Assert::new_from(input, name, code))
    }
}

impl TokenConsumer for Content {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match self {
            Content::None => panic!("unreachable state detected"),
            Content::Body(body) => body.accept_token(token),
            Content::Assert(assert) => assert.accept_token(token),
        }
    }
}

impl TokenGenerator for Content {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        match self {
            Content::None => { /* Nothing to generate */ }
            Content::Body(body) => body.generate_tokens(collector),
            Content::Assert(assert) => assert.generate_tokens(collector),
        }
    }
}
