use std::{marker::PhantomData, mem::take};

use proc_macro2::TokenTree;

use crate::{
    consumer::TokenConsumer, error::*, generator::TokenGenerator, token::Token, token_is::TokenIs,
};

pub(crate) enum Rule<T> {
    Uninitialized,
    Open(Open<T>),
    Closed(Closed<T>),
    OpenError(OpenError<T>),
    ClosedError(ClosedError<T>),
}

impl<T> Rule<T> {
    pub(crate) fn found(value: T) -> Self {
        Self::Open(Open(Some(value)))
    }

    pub(crate) fn open_error(start_point: TokenTree, msg: String) -> Self {
        Self::OpenError(OpenError::new(start_point, msg))
    }
}

impl<T: TokenConsumer> TokenConsumer for Rule<T> {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match self {
            Rule::Uninitialized => TokenIs::failed(
                format!(
                    "rule not yet initialised but received unexpected token {}",
                    token
                ),
                token,
            ),

            Rule::Open(open) => open.accept_token(token),
            Rule::Closed(closed) => closed.accept_token(token),
            Rule::OpenError(error) => error.accept_token(token),
            Rule::ClosedError(error) => error.accept_token(token),
        }
    }
}

impl<T: TokenGenerator> TokenGenerator for Rule<T> {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        match self {
            Rule::Uninitialized => {
                /* nothing to generate */
                return;
            }
            Rule::Open(_) => panic!("logic error, this shouldn't be a possible state"),
            Rule::Closed(closed) => closed.generate_tokens(collector),
            Rule::OpenError(_) => panic!("logic error, this shouldn't be a possible state"),
            Rule::ClosedError(closed_error) => closed_error.generate_tokens(collector),
        }
    }
}

pub(crate) struct Open<T>(Option<T>);

impl<T> Open<T> {
    pub(crate) fn close(&mut self) -> Rule<T> {
        Rule::Closed(Closed(self.0.take().unwrap()))
    }
}

impl<T: TokenConsumer> TokenConsumer for Open<T> {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        self.0.as_mut().unwrap().accept_token(token)
    }
}

impl<T: TokenGenerator> TokenGenerator for Open<T> {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        match self.0.take() {
            Some(mut open) => open.generate_tokens(collector),
            None => { /* nothing to generate */ }
        }
    }
}

pub(crate) struct Closed<T>(T);
impl<T: TokenGenerator> TokenGenerator for Closed<T> {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        self.0.generate_tokens(collector)
    }
}

impl<T> TokenConsumer for Closed<T> {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match token {
            Token::EndOfStream => TokenIs::Consumed,
            _ => TokenIs::Rejected(token),
        }
    }
}

pub(crate) struct OpenError<T> {
    initial_token: TokenTree,
    error: crate::error::Error,
    phantom: PhantomData<T>,
}

impl<T> OpenError<T> {
    pub(crate) fn new(initial_token: TokenTree, msg: String) -> Self {
        Self {
            error: Error::new(msg, &initial_token),
            initial_token,
            phantom: PhantomData,
        }
    }
    pub(crate) fn close(&mut self, value: Option<T>) -> Rule<T> {
        Rule::ClosedError(ClosedError::new(self, value))
    }
}

impl<T> TokenConsumer for OpenError<T> {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match token {
            Token::Token(token_tree) => self.error.push(token_tree),
            Token::EndOfStream => {}
        };
        TokenIs::Consumed
    }
}

pub(crate) struct ClosedError<T> {
    initial_token: TokenTree,
    error: crate::error::Error,
    next: Option<T>,
}

impl<T> ClosedError<T> {
    fn new(error: &mut OpenError<T>, value: Option<T>) -> Self {
        Self {
            initial_token: error.initial_token.clone(),
            error: take(&mut error.error),
            next: value,
        }
    }
}

impl<T: TokenGenerator> TokenGenerator for ClosedError<T> {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        self.error.generate_tokens(collector);
        collector.push(self.initial_token.clone());
        if let Some(next) = &mut self.next {
            next.generate_tokens(collector);
        }
    }
}

impl<T> TokenConsumer for ClosedError<T> {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        TokenIs::Rejected(token)
    }
}
