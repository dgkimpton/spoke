use proc_macro2::TokenTree;

use crate::{error::*, name::Name, span_source::*, token_helpers::*};

enum AssertState {
    Start,
    Left,
    AssertionType,
    Right,
    CustomMessage,
    Panic,
    Done,
}

type CodeBlock = Vec<TokenTree>;

struct Eq {
    span: Span,
}

enum AssertType {
    Eq(Eq),
}

pub(crate) struct Assert {
    state: AssertState,
    name: Name,
    code: Vec<TokenTree>,
    left_expression: CodeBlock,
    assert_type: Option<AssertType>,
    right_expression: CodeBlock,
    message: Option<String>,
    custom_exp_args: Vec<CodeBlock>,
}

impl Assert {
    pub(crate) fn new(name: Name, parent_code: Vec<TokenTree>) -> Self {
        Self {
            state: AssertState::Start,
            name,
            code: parent_code,
            left_expression: Vec::new(),
            assert_type: None,
            right_expression: Vec::new(),
            message: None,
            custom_exp_args: Vec::new(),
        }
    }
    pub(crate) fn accept_token(&mut self, tok: &proc_macro2::TokenTree) -> CompileResult<bool> {
        match self.state {
            AssertState::Start => match &tok {
                TokenTree::Punct(punct) if punct.as_char() == '$' => CompileError::err(
                    tok,
                    "found the $ indicating an assertion before the left hand side (actual value) of the assertion was provided",
                ),
                _ => {
                    self.state = AssertState::Left;
                    self.left_expression.push(tok.clone());
                    Ok(true)
                }
            },
            AssertState::Left => match &tok {
                TokenTree::Punct(punct) if punct.as_char() == '$' => {
                    self.state = AssertState::AssertionType;
                    Ok(true)
                }
                _ => {
                    self.left_expression.push(tok.clone());
                    Ok(true)
                }
            },
            AssertState::AssertionType => match tok {
                TokenTree::Ident(ident) => match ident.to_string().as_str() {
                    "eq" => {
                        self.assert_type = Some(AssertType::Eq(Eq::new(tok)));
                        self.state = AssertState::Right;
                        Ok(true)
                    }
                    _ => CompileError::err(
                        tok,
                        format!("expected an assertion type, but found {ident}"),
                    ),
                },
                token => CompileError::err(
                    tok,
                    format!("expected an assertion type, but found {token}"),
                ),
            },
            AssertState::Right => match &tok {
                TokenTree::Punct(punct) if punct.as_char() == ';' => {
                    self.state = AssertState::Done;
                    Ok(true)
                }
                TokenTree::Punct(punct) if punct.as_char() == '$' => {
                    self.state = AssertState::CustomMessage;
                    Ok(true)
                }
                _ => {
                    self.right_expression.push(tok.clone());
                    Ok(true)
                }
            },
            AssertState::CustomMessage => Ok(true),
            AssertState::Panic => Ok(true),
            AssertState::Done => {
                // don't want this token, we are already done
                Ok(false)
            }
        }
    }

    pub(crate) fn generate_tokens(&mut self) -> CompileResult<Vec<TokenTree>> {
        match &mut self.assert_type {
            Some(assert_type) => Ok(vec![
                punct('#'),
                bracketed([ident("test", *self.name.span())]),
                ident("fn", *self.name.span()),
                ident(self.name.function_name()?.as_str(), *self.name.span()),
                parenthesised([]),
                braced(
                    std::mem::take(&mut self.code)
                        .into_iter()
                        .chain(match assert_type {
                            AssertType::Eq(eq) => eq
                                .generate_tokens(
                                    std::mem::take(&mut self.left_expression),
                                    std::mem::take(&mut self.right_expression),
                                )?
                                .into_iter(),
                        }),
                ),
            ]),
            None => {
                return CompileError::err(
                    self.name.span(),
                    format!(
                        "no body or assertion found for named test {}",
                        self.name.full_name()?
                    ),
                );
            }
        }
    }
}

impl Eq {
    fn generate_tokens(
        &self,
        left: Vec<TokenTree>,
        right: Vec<TokenTree>,
    ) -> CompileResult<Vec<TokenTree>> {
        Ok(vec![
            ident("assert_eq", self.span),
            punct('!'),
            parenthesised(
                left.into_iter()
                    .chain(vec![punct(',')])
                    .chain(right.into_iter())
                    .collect::<Vec<TokenTree>>(),
            ),
            punct(';'),
        ])
    }

    fn new(sp: &impl SpanSource) -> Self {
        Self { span: sp.span() }
    }
}
