use std::mem::take;

use proc_macro2::TokenTree;

use crate::{generator::*, span_source::*, token::Token, token_helpers::*};

pub(crate) struct Error {
    span: proc_macro2::Span,
    msg: String,
    followed_by: Vec<TokenTree>,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            span: Span::call_site(),
            msg: String::new(),
            followed_by: Vec::new(),
        }
    }
}

impl Error {
    pub(crate) fn new(msg: String, span: &impl SpanSource) -> Self {
        Self {
            span: span.span(),
            msg,
            followed_by: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, token: TokenTree) {
        self.followed_by.push(token)
    }
}

impl TokenGenerator for Error {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        collector.extend(
            [
                ident("compile_error", self.span),
                punct('!'),
                parenthesised(vec![lit_string(take(&mut self.msg))]),
                punct(';'),
            ]
            .into_iter()
            .chain(take(&mut self.followed_by).into_iter()),
        );
    }
}

pub(crate) struct ProcessingError {
    span: proc_macro2::Span,
    msg: String,
}

impl ProcessingError {
    pub(crate) fn new(msg: String, source: Token) -> Self {
        Self {
            span: source.span(),
            msg,
        }
    }
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl TokenGenerator for ProcessingError {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        collector.extend([
            ident("compile_error", self.span),
            punct('!'),
            parenthesised(vec![lit_string(take(&mut self.msg))]),
            punct(';'),
        ]);
    }
}

// pub(crate) type CompileResult<T> = Result<T, CompileError>;

// #[derive(Debug)]
// pub(crate) struct CompileError {
//     span: proc_macro2::Span,
//     msg: String,
//     pub(crate) is_complete: bool,
// }

// impl PartialEq for CompileError {
//     fn eq(&self, other: &Self) -> bool {
//         self.span.start() == other.span.start()
//             && self.span.end() == other.span.end()
//             && self.msg == other.msg
//     }
// }

// impl CompileError {
//     pub(crate) fn new(span: proc_macro2::Span, msg: String) -> Self {
//         Self {
//             span,
//             msg,
//             is_complete: false,
//         }
//     }

//     pub(crate) fn err<TSuccess>(
//         span: &impl SpanSource,
//         msg: impl Into<String>,
//     ) -> Result<TSuccess, Self> {
//         Err(Self::new(span.span(), msg.into()))
//     }

//     pub(crate) fn generate_tokens(self) -> Vec<TokenTree> {
//         vec![
//             ident("compile_error", self.span),
//             punct('!'),
//             parenthesised(vec![lit_string(self.msg)]),
//             punct(';'),
//         ]
//     }
// }

// impl std::fmt::Display for CompileError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "error parsing tests : {}", self.msg)
//     }
// }
