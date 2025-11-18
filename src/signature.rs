use std::mem::take;

use proc_macro2::{Delimiter, Span, TokenTree};

use crate::{
    code_block::CodeBlock, consumer::TokenConsumer, generator::TokenGenerator, name::*,
    named::Named, rule::*, span_source::SpanSource, token::Token, token_is::TokenIs,
};

pub(crate) struct Signature {
    anchor_span: Span,
    name_factory: NameFactory,
    named: Rule<Named>,
    code: CodeBlock,
}

impl Signature {
    pub(crate) fn with_parent(
        anchor_span: &impl SpanSource,
        name_factory: NameFactory,
        code: CodeBlock,
    ) -> Self {
        Self {
            anchor_span: anchor_span.span(),
            name_factory,
            code,
            named: Rule::Uninitialized,
        }
    }
    pub(crate) fn new(span: &impl SpanSource, parent_name_factory: NameFactory) -> Self {
        Self::with_parent(span, parent_name_factory, CodeBlock::new())
    }
}

impl TokenConsumer for Signature {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match &mut self.named {
            Rule::Uninitialized => match token {
                Token::Token(token_tree) => match litrs::StringLit::try_from(&token_tree) {
                    Ok(literal) => {
                        self.named = Rule::found(Named::new(
                            self.name_factory
                                .make_name_from_str(&token_tree, literal.value()),
                            take(&mut self.code),
                        ));

                        TokenIs::Consumed
                    }

                    Err(error) => {
                        let message = format!(
                            "expected a test case name in quotes following the $ inside {}, but found `{}`\nERROR: {}",
                            self.name_factory.qualified_name(&self.anchor_span),
                            token_tree.to_string(),
                            error
                        );
                        self.named = Rule::open_error(token_tree, message);

                        TokenIs::Consumed
                    }
                },

                Token::EndOfStream => TokenIs::failed_at_end(format!(
                    "unexpected end of stream before test name in {}",
                    self.name_factory.qualified_name(&self.anchor_span)
                )),
            },

            Rule::Open(open) => match open.accept_token(token) {
                TokenIs::Rejected(token) => {
                    self.named = open.close();
                    TokenIs::Rejected(token)
                }
                result => result,
            },

            Rule::OpenError(error) => {
                match token {
                    Token::Token(TokenTree::Group(group))
                        if group.delimiter() == Delimiter::Brace =>
                    {
                        // assume this is probably the body of the test
                        let (named, processing_error) = Named::new_from(
                            self.name_factory.make_name_from_str(&group, "missing_name"),
                            take(&mut self.code),
                            group,
                        );

                        self.named = error.close(Some(named));

                        if let Some(processing_error) = processing_error {
                            TokenIs::FailedProcessing(processing_error)
                        } else {
                            TokenIs::Consumed
                        }
                    }

                    Token::Token(TokenTree::Punct(punct)) if punct.as_char() == ';' => {
                        // assume this is probably the end of the test and it was simply invalid
                        self.named = error.close(None);
                        TokenIs::Consumed
                    }

                    other_token => error.accept_token(other_token),
                }
            }

            rule => rule.accept_token(token),
        }
    }
}

impl TokenGenerator for Signature {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        self.named.generate_tokens(collector)
    }
}
