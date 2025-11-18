use crate::{
    consumer::*,
    generator::{IntoVecTokens, TokenGenerator},
    name::NameFactory,
    signature::Signature,
    token::Token,
    token_helpers::*,
    token_is::TokenIs,
};

pub(crate) struct Suite {
    name_factory: NameFactory,
    preamble: Vec<TokenTree>,
    signatures: Vec<Signature>,
    partial_signature: Option<Signature>,
}

impl Suite {
    pub(crate) fn new(name_factory: NameFactory) -> Self {
        Self {
            name_factory,
            preamble: Vec::new(),
            signatures: Vec::new(),
            partial_signature: None,
        }
    }
}

impl TokenConsumer for Suite {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        if let Some(signature) = &mut self.partial_signature {
            match signature.accept_token(token) {
                TokenIs::Rejected(rejected_token) => {
                    // test rejected the input which means that this test is now complete.
                    self.signatures.push(
                        self.partial_signature
                            .take()
                            .expect("the partial test to always exist here"),
                    );

                    // look for more signatures starting with the previously rejected
                    self.accept_token(rejected_token)
                }

                result => result,
            }
        } else {
            match token {
                Token::Token(TokenTree::Punct(punct)) if punct.as_char() == '$' => {
                    self.partial_signature =
                        Some(Signature::new(&punct, self.name_factory.clone()));
                    TokenIs::Consumed
                }

                Token::Token(token_tree) => {
                    self.preamble.push(token_tree);
                    TokenIs::Consumed
                }

                Token::EndOfStream => {
                    if let Some(last_signature) = &mut self.partial_signature {
                        match last_signature.accept_token(Token::EndOfStream) {
                            TokenIs::Rejected(rejected_token) => {
                                self.signatures.push(
                                    /* note: we could take in the if let above, but then a failure in accept would lose the signature completely */
                                    self.partial_signature
                                        .take()
                                        .expect("the last partial test to always exist here"),
                                );

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

impl TokenGenerator for Suite {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        if self.signatures.is_empty() && self.preamble.is_empty() {
            return;
        }

        let call_site = Span::call_site();

        collector.extend([
            punct('#'),
            bracketed([
                ident("cfg", call_site),
                parenthesised([ident("test", call_site)]),
            ]),
            punct('#'),
            bracketed([
                ident("allow", call_site),
                parenthesised([ident("unused_mut", call_site)]),
            ]),
            punct('#'),
            bracketed([
                ident("allow", call_site),
                parenthesised([ident("unused_variables", call_site)]),
            ]),
            ident("mod", call_site),
            ident("spoketest", call_site),
            braced_stream(self.generate_suite()),
        ]);
    }
}

impl Suite {
    fn generate_suite(&mut self) -> Stream {
        let mut content = Stream::new();

        content.extend(std::mem::take(&mut self.preamble).into_iter());

        for test in &mut self.signatures {
            content.extend(test.into_vec());
        }

        content
    }
}
