use std::mem::take;

use proc_macro2::TokenTree;

use crate::{
    code_block::CodeBlock, consumer::TokenConsumer, generator::TokenGenerator, name::Name,
    operand::Operand, rule::Rule, token::Token, token_is::TokenIs,
};

pub(crate) struct Assert {
    name: Option<Name>,
    code: CodeBlock,
    left: CodeBlock,
    operand: Rule<Operand>,
}

impl Assert {
    pub(crate) fn new_from(first_token: TokenTree, name: Name, code: CodeBlock) -> Self {
        let mut left = CodeBlock::new();
        left.push(first_token);

        Self {
            name: Some(name),
            code,
            left,
            operand: Rule::Uninitialized,
        }
    }
}

impl TokenConsumer for Assert {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match &mut self.operand {
            Rule::Uninitialized => match token {
                Token::Token(TokenTree::Punct(punct)) if punct.as_char() == '$' => {
                    self.operand = Rule::found(Operand::new(
                        self.name.take().expect("name should exist"),
                        take(&mut self.code),
                        take(&mut self.left),
                    ));
                    TokenIs::Consumed
                }

                Token::Token(other) => {
                    self.left.push(other);
                    TokenIs::Consumed
                }

                Token::EndOfStream => TokenIs::failed_at_end(format!(
                    "unexpected end of stream in definition of test assertion before definition of assertion type in {}",
                    self.name.as_ref().expect("name exists").full_name()
                )),
            },
            Rule::Open(open) => match open.accept_token(token) {
                TokenIs::FailedProcessing(error) => TokenIs::FailedProcessing(error),
                TokenIs::Rejected(rejected) => TokenIs::Rejected(rejected),
                TokenIs::Consumed => TokenIs::Consumed,
                TokenIs::ConsumedAndFinished => {
                    self.operand = open.close();
                    TokenIs::ConsumedAndFinished
                }
            },
            Rule::Closed(closed) => closed.accept_token(token),
            Rule::OpenError(error) => error.accept_token(token),
            Rule::ClosedError(error) => error.accept_token(token),
        }
    }
}

impl TokenGenerator for Assert {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        self.operand.generate_tokens(collector)
    }
}
