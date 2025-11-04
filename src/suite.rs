use proc_macro2::{TokenStream, TokenTree};

use crate::{error::*, name::*, signature::TestSignature, token_helpers::*};

pub(crate) fn process(input: TokenStream) -> CompileResult<Vec<TokenTree>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let mut suite = TestSuite::new(NameFactory::new());

    for tok in input {
        if !suite.accept_token(&tok)? {
            return CompileError::err(
                &tok,
                format!("unexpected token `{}` in combitest macro", tok),
            );
        }
    }

    suite.generate_tokens()
}

enum TestSuiteState {
    Preamble,
    TestSpecification,
}

pub(crate) struct TestSuite {
    state: TestSuiteState,
    name_factory: NameFactory,
    preamble: Vec<TokenTree>,
    tests: Vec<TestSignature>,
    partial_test: Option<TestSignature>,
}

impl TestSuite {
    pub(crate) fn new(name_factory: NameFactory) -> Self {
        Self {
            state: TestSuiteState::Preamble,
            name_factory,
            preamble: Vec::new(),
            tests: Vec::new(),
            partial_test: None,
        }
    }

    pub(crate) fn accept_token(&mut self, tok: &TokenTree) -> CompileResult<bool> {
        match self.state {
            TestSuiteState::Preamble => match tok {
                TokenTree::Punct(punct) if punct.as_char() == '$' => {
                    self.partial_test = Some(TestSignature::new(
                        tok.span(),
                        self.name_factory.clone(),
                        vec![],
                    ));
                    self.state = TestSuiteState::TestSpecification;
                    Ok(true)
                }
                _ => {
                    self.preamble.push(tok.clone());
                    Ok(true)
                }
            },
            TestSuiteState::TestSpecification => {
                if self.partial_test.as_mut().unwrap().accept_token(tok)? {
                    Ok(true)
                } else {
                    // test rejected the input without error which means that
                    // the test is now complete.
                    self.tests.push(self.partial_test.take().unwrap());
                    self.state = TestSuiteState::Preamble;

                    // back to the start looking for more tests
                    self.accept_token(tok)
                }
            }
        }
    }
    
    pub(crate) fn generate_tokens(&mut self) -> CompileResult<Vec<TokenTree>> {
        if self.partial_test.is_some() {
            self.tests.push(self.partial_test.take().unwrap());
        }

        if self.tests.is_empty() && self.preamble.is_empty() {
            return Ok(vec![]);
        }

        let mut test_tokens = Vec::new();

        for test in &mut self.tests {
            let mut tokens = test.generate_tokens()?;
            test_tokens.append(&mut tokens);
        }

        Ok(vec![
            punct('#'),
            bracketed([
                ident("cfg", Span::call_site()),
                parenthesised([ident("test", Span::call_site())]),
            ]),

            punct('#'),
            bracketed([
                ident("allow", Span::call_site()),
                parenthesised([ident("unused_mut", Span::call_site())]),
            ]),
            
            punct('#'),
            bracketed([
                ident("allow", Span::call_site()),
                parenthesised([ident("unused_variables", Span::call_site())]),
            ]),
            
            ident("mod", Span::call_site()),
            ident("combitests", Span::call_site()),
            braced(self.preamble.drain(..).chain(test_tokens)),
        ])
    }
}
