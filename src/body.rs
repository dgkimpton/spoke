use proc_macro2::TokenTree;

use crate::{
    code_block::CodeBlock, consumer::*, error::ProcessingError, generator::TokenGenerator, name::*,
    signature::Signature, token::Token, token_helpers::*, token_is::TokenIs,
};

pub(crate) struct Body {
    name: Name,
    child_name_factory: NameFactory,
    code: CodeBlock,
    children: Vec<Signature>,
    partial_child: Option<Box<Signature>>,
}

impl Body {
    pub(crate) fn new(name: Name, parent_code: CodeBlock) -> Self {
        Self {
            child_name_factory: name.make_factory(),
            name,
            code: parent_code,
            children: Vec::new(),
            partial_child: None,
        }
    }

    pub(crate) fn new_from(
        input: impl IterableTokens,
        name: Name,
        code: CodeBlock,
    ) -> (Self, Option<ProcessingError>) {
        input.process_into(Self::new(name,  code))
    }
}

impl TokenConsumer for Body {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        if let Some(child) = &mut self.partial_child {
            match child.accept_token(token) {
                TokenIs::Rejected(token) => {
                    // test rejected the input which means that the test is now complete.
                    self.children.push(*self.partial_child.take().unwrap());
                    self.accept_token(token)
                }
                TokenIs::Consumed => todo!(),
                TokenIs::ConsumedAndFinished => {
                    
                },
                TokenIs::FailedProcessing(processing_error) => todo!(),
            }
        } else {
            match token {
                Token::Token(TokenTree::Punct(punct)) if punct.as_char() == '$' => {
                    self.partial_child = Some(Box::new(Signature::with_parent(
                        &punct,
                        self.child_name_factory.clone(),
                        self.code.clone(),
                    )));
                    TokenIs::Consumed
                }

                Token::Token(other_token) => {
                    self.code.push(other_token);
                    TokenIs::Consumed
                }

                Token::EndOfStream => {
                    if let Some(last_child) = &mut self.partial_child {
                        match last_child.accept_token(token) {
                            TokenIs::Rejected(rejected_token) => {
                                self.children.push(*
                                    /* note: we could take in the if let above, but then a failure in accept would lose the child completely */
                                    self.partial_child
                                        .take()
                                        .expect("the last partial child to always exist here"),);

                                self.accept_token(rejected_token)
                            }
                            result => result,
                        }
                    } else {
                        TokenIs::Rejected(Token::EndOfStream)
                    }
                }
            }
        }
    }
}

impl TokenGenerator for Body {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        if self.children.is_empty() {
            collector.extend([
                punct('#'),
                bracketed([ident("test", *self.name.span())]),
                ident("fn", *self.name.span()),
                ident(self.name.function_name().as_str(), *self.name.span()),
                parenthesised([]),
                braced(self.code.take()),
            ])
        }
    }
}
