pub(crate) use proc_macro2::TokenTree;

use crate::{
    error::ProcessingError, token::Token, token_helpers::IterableTokens, token_is::TokenIs,
};

pub(crate) trait TokenConsumer {
    fn accept_token(&mut self, token: Token) -> TokenIs;
}

pub(crate) trait TokenStreamExt {
    fn process_into<T: TokenConsumer>(self, consumer: T) -> (T, Option<ProcessingError>);
}

impl<Consumer: IterableTokens> TokenStreamExt for Consumer {
    fn process_into<T: TokenConsumer>(self, mut consumer: T) -> (T, Option<ProcessingError>) {
        for token in self
            .into_iter()
            .map(|tok| Token::Token(tok))
            .chain([Token::EndOfStream])
        {
            match consumer.accept_token(token) {
                TokenIs::Consumed => continue,
                TokenIs::FailedProcessing(error) => return (consumer, Some(error)),
                TokenIs::Rejected(tok) => {
                    match tok {
                        Token::EndOfStream => {
                            break; // consumers are expected to reject the end of stream
                        }

                        Token::Token(token) => {
                            return (
                                consumer,
                                Some(ProcessingError::new(
                                    format!(
                                        "unexpected token in input stream :: {}",
                                        token.to_string()
                                    ),
                                    Token::Token(token),
                                )),
                            );
                        }
                    }
                }
            }
        }
        (consumer, None)
    }
}
