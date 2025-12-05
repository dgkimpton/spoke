use std::fmt::Display;

use proc_macro2::{Span, TokenTree};

use crate::span_source::*;

#[derive(Debug)]
pub(crate) enum Token {
    Token(TokenTree),
    EndOfStream,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Token(token_tree) => token_tree.fmt(f),
            Token::EndOfStream => write!(f, "EndOfStream"),
            Token::EndOfGroup => write!(f, "EndOfGroup"),
        }
    }
}

impl SpanSource for Token {
    fn span(&self) -> Span {
        match self {
            Token::Token(token_tree) => token_tree.span(),
            Token::EndOfStream => Span::call_site(),
            Token::EndOfGroup =>  Span::call_site(),
        }
    }
}
