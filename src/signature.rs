use proc_macro2::{Delimiter, Span, TokenTree};

use crate::{
    asserts::Assert,
    body::TestBody,
    error::*,
    name::*,
    token_helpers::{IterableTokens, TokenTreeExtensions},
};

enum TestSignatureState {
    ExpectsName,
    ExpectsBodyOrAssert,
    HandlingAssert,
    IsDone,
}

enum Content {
    Body(TestBody),
    Assert(Assert),
}

pub(crate) struct TestSignature {
    state: TestSignatureState,
    anchor_span: Span,
    parent_name_factory: NameFactory,
    parent_code: Vec<TokenTree>,
    name: Option<Name>,
    child_name_factory: Option<NameFactory>,
    content: Option<Content>,
}

impl TestSignature {
    pub(crate) fn new(
        anchor_span: Span,
        parent_name_factory: NameFactory,
        parent_code: impl IterableTokens,
    ) -> Self {
        Self {
            state: TestSignatureState::ExpectsName,
            anchor_span,
            parent_name_factory,
            parent_code: parent_code.into_iter().collect(),
            name: None,
            child_name_factory: None,
            content: None,
        }
    }

    pub(crate) fn accept_token(&mut self, tok: &proc_macro2::TokenTree) -> CompileResult<bool> {
        match self.state {
            TestSignatureState::ExpectsName => {
                println!("ExpectsName {}", &tok);
                let name = self.parent_name_factory.make_name(
                    tok,
                    tok.expect_string_literal("expected name of the testcase")
                        .map_err(|e| {
                            CompileError::new(
                                tok.span(),
                                format!(
                                    "in {}  :: {} :: {}",
                                    self.parent_name_factory
                                        .qualified_name(tok)
                                        .unwrap_or("test suite".into()),
                                    e,
                                    tok.to_string()
                                ),
                            )
                        })?,
                );

                self.child_name_factory = Some(name.make_factory());
                self.name = Some(name);
                self.state = TestSignatureState::ExpectsBodyOrAssert;
                Ok(true)
            }

            TestSignatureState::ExpectsBodyOrAssert => {
                println!("ExpectsBodyOrAssert {}", &tok);
                match tok {
                    TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                        let mut body = TestBody::new(
                            self.name.take().unwrap(),
                            self.child_name_factory.take().unwrap(),
                            std::mem::take(&mut self.parent_code),
                        );

                        for token in group.stream() {
                            if !body.accept_token(&token)? {
                                return CompileError::err(
                                    &token,
                                    "test body rejected a token. Please contact combitest project and provide the code sample because this shouldn't happen",
                                );
                            }
                        }
                        self.content = Some(Content::Body(body));
                        self.state = TestSignatureState::IsDone;
                        Ok(true)
                    }

                    _ => {
                        self.state = TestSignatureState::HandlingAssert;
                        self.content = Some(Content::Assert(Assert::new(
                            self.name.take().unwrap(),
                            std::mem::take(&mut self.parent_code),
                        )));
                        self.accept_token(tok)
                    }
                }
            }

            TestSignatureState::HandlingAssert => {
                println!("HandlingAssert {}", &tok);
                match &mut self.content {
                    Some(content) => match content {
                        Content::Body(_) => panic!(
                            "shouldn't be possible to be handling an assert if a body has been found"
                        ),
                        Content::Assert(assert) => {
                            if !assert.accept_token(tok)? {
                                self.state = TestSignatureState::IsDone;
                                Ok(false)
                            } else {
                                Ok(true)
                            }
                        }
                    },
                    None => panic!(
                        "shouldn't be possible to be handling an assert until an assert has been signaled"
                    ),
                }
            }

            TestSignatureState::IsDone => {
                println!("IsDone {}", &tok);
                // don't want this token, we are already done
                Ok(false)
            }
        }
    }

    pub(crate) fn generate_tokens(&mut self) -> CompileResult<Vec<proc_macro2::TokenTree>> {
        match &mut self.content {
            Some(content) => match content {
                Content::Body(body) => body.generate_tokens(),
                Content::Assert(assert) => assert.generate_tokens(),
            },
            None => match &self.name {
                Some(name) => CompileError::err(
                    name.span(),
                    format!(
                        "test case specified without a body at `{}`. expected a test body in braces or a valid assertion",
                        name.full_name()?
                    ),
                ),
                None => {
                    if self.parent_name_factory.has_parent() {
                        CompileError::err(
                            &self.anchor_span,
                            format!(
                                "test case specified within `{}` without a name. expected a test name in quotes",
                                self.parent_name_factory.qualified_name(&self.anchor_span)?
                            ),
                        )
                    } else {
                        CompileError::err(
                            &self.anchor_span,
                            "expected a test name in quotes".to_string(),
                        )
                    }
                }
            },
        }
    }
}
