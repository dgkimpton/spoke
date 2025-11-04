use proc_macro2::{Delimiter, Span, TokenTree};

use crate::{
    error::*,
    name::*,
    body::TestBody,
    token_helpers::{IterableTokens, TokenTreeExtensions},
};

enum TestSignatureState {
    ExpectsName,
    ExpectsBody,
    IsDone,
}

pub(crate) struct TestSignature {
    state: TestSignatureState,
    anchor_span: Span,
    parent_name_factory: NameFactory,
    parent_code: Vec<TokenTree>,
    name: Option<Name>,
    child_name_factory: Option<NameFactory>,
    body: Option<TestBody>,
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
            body: None,
        }
    }

    pub(crate) fn accept_token(&mut self, tok: &proc_macro2::TokenTree) -> CompileResult<bool> {
        match self.state {
            TestSignatureState::ExpectsName => {
                let name = self.parent_name_factory.make_name(
                    tok,
                    tok.expect_string_literal("expected name of the testcase")?,
                );

                self.child_name_factory = Some(name.make_factory());
                self.name = Some(name);
                self.state = TestSignatureState::ExpectsBody;
                Ok(true)
            }

            TestSignatureState::ExpectsBody => match tok {
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
                    self.body = Some(body);
                    self.state = TestSignatureState::IsDone;
                    Ok(true)
                }
                _ => CompileError::err(
                    tok,
                    format!(
                        "expected to find the body of the test in braces, but :: got `{}`",
                        tok
                    ),
                ),
            },

            TestSignatureState::IsDone => {
                // don't want this token, we are already done
                Ok(false)
            }
        }
    }

    pub(crate) fn generate_tokens(&mut self) -> CompileResult<Vec<proc_macro2::TokenTree>> {
        match &mut self.body {
            None => match &self.name {
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
                Some(name) => CompileError::err(
                    name.span(),
                    format!(
                        "test case specified without a body at `{}`. expected a test body in braces",
                        name.full_name()?
                    ),
                ),
            },
            Some(body) => body.generate_tokens(),
        }
    }
}
