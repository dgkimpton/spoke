use proc_macro2::TokenTree;

use crate::{
    code_block::CodeBlock, consumer::TokenConsumer, generator::TokenGenerator, token::Token, token_is::TokenIs,
};

pub(crate) struct Right {
    code: CodeBlock,
}

impl Right {
    pub(crate) fn new_from(first_token: TokenTree) -> Self {
        let mut code = CodeBlock::new();
        code.push(first_token);

        Self { code }
    }
}

impl TokenConsumer for Right {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match token {
            Token::Token(TokenTree::Punct(punct)) if punct.as_char() == ';' => {
                TokenIs::ConsumedAndFinished
            }
            Token::Token(other) => {
                self.code.push(other);
                TokenIs::Consumed
            }
            Token::EndOfStream => {
                TokenIs::failed_at_end(format!("expected end of stream in unterminated code block"))
            }
        }
    }
}

impl TokenGenerator for Right {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        collector.extend(self.code.take());
    }
}
