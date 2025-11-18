use proc_macro2::Span;

use crate::{
    code_block::CodeBlock, consumer::*, generator::*, name::Name, right::Right, rule::Rule,
    span_source::SpanSource, token::Token, token_helpers::*, token_is::TokenIs,
};

pub(crate) struct EqualityOperand {
    anchor_span: Span,
    name: Name,
    code: CodeBlock,
    left: CodeBlock,
    right: Rule<Right>,
}

impl EqualityOperand {
    pub(crate) fn new(
        name: Name,
        code: CodeBlock,
        anchor_span: &impl SpanSource,
        left: CodeBlock,
    ) -> Self {
        Self {
            name,
            code,
            anchor_span: anchor_span.span(),
            left,
            right: Rule::Uninitialized,
        }
    }
}

impl TokenConsumer for EqualityOperand {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match &mut self.right {
            Rule::Uninitialized => match token {
                Token::Token(TokenTree::Punct(punct)) if punct.as_char() == ';' => TokenIs::failed(
                    format!("expected an expression after $eq, but found `;`"),
                    Token::Token(TokenTree::Punct(punct)),
                ),

                Token::EndOfStream => TokenIs::failed_at_end(format!(
                    "unexpected end of stream in equality assertion, expected an expression "
                )),
                Token::Token(other) => {
                    self.right = Rule::found(Right::new_from(other));
                    TokenIs::Consumed
                }
            },
            Rule::Open(open) => match open.accept_token(token) {
                TokenIs::Rejected(rejected) => {
                    self.right = open.close();
                    TokenIs::Rejected(rejected)
                }
                result => result,
            },
            Rule::Closed(closed) => closed.accept_token(token),
            Rule::OpenError(open_error) => open_error.accept_token(token),
            Rule::ClosedError(closed_error) => closed_error.accept_token(token),
        }
    }
}

impl TokenGenerator for EqualityOperand {
    fn generate_tokens(&mut self, collector: &mut Vec<proc_macro2::TokenTree>) {
        collector.extend([
            ident("assert_eq", self.anchor_span),
            punct('!'),
            parenthesised(
                self.left
                    .take()
                    .into_iter()
                    .chain([punct(',')])
                    .chain(self.right.into_vec()),
            ),
        ]);
    }
}
