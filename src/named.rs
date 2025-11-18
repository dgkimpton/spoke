use std::mem::take;

use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::TokenTree;

use crate::code_block::CodeBlock;
use crate::error::ProcessingError;
use crate::generator::*;
use crate::token::Token;
use crate::token_is::TokenIs;
use crate::{consumer::*, content::Content, name::*, rule::Rule};

pub(crate) struct Named {
    name: Option<Name>,
    code: CodeBlock,
    content: Rule<Content>,
}

impl Named {
    pub(crate) fn new(name: Name, code: CodeBlock) -> Self {
        Self {
            name: Some(name),
            content: Rule::Uninitialized,
            code,
        }
    }

    pub(crate) fn new_from(
        name: Name,
        code: CodeBlock,
        input: Group,
    ) -> (Self, Option<ProcessingError>) {
        let (content, error) = Content::body(input, name, code);
        (
            Self {
                content: Rule::found(content),
                code: CodeBlock::default(),
                name: None,
            },
            error,
        )
    }
}

impl TokenConsumer for Named {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match &mut self.content {
            Rule::Uninitialized => match token {
                Token::Token(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                    let (body, error) =
                        Content::body(group, self.name.take().unwrap(), take(&mut self.code));
                    self.content = Rule::found(body);

                    if let Some(error) = error {
                        TokenIs::FailedProcessing(error)
                    } else {
                        TokenIs::Consumed
                    }
                }

                Token::Token(other) => {
                    self.content = Rule::found(Content::assert(
                        other,
                        self.name.take().unwrap(),
                        take(&mut self.code),
                    ));
                    TokenIs::Consumed
                }

                Token::EndOfStream => {
                    self.content = Rule::found(Content::None);
                    TokenIs::failed_at_end(format!(
                        "reached end of input before reaching the end of the test definition in {}",
                        self.name
                            .as_ref()
                            .expect("name should already exist")
                            .full_name()
                    ))
                }
            },

            Rule::Open(content) => match content.accept_token(token) {
                TokenIs::Rejected(token) => {
                    self.content = content.close();
                    TokenIs::Rejected(token)
                }
                result => result,
            },

            rule => rule.accept_token(token),
        }
    }
}

impl TokenGenerator for Named {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        self.content.generate_tokens(collector);
    }
}
