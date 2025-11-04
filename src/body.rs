use proc_macro2::TokenTree;

use crate::{
    error::*,
    name::{Name, NameFactory},
    signature::TestSignature,
    token_helpers::*,
};

enum TestBodyState {
    Code,
    ChildTest,
}

pub(crate) struct TestBody {
    state: TestBodyState,
    name: Name,
    child_name_factory: NameFactory,
    code: Vec<TokenTree>,
    children: Vec<TestSignature>,
    partial_child: Option<Box<TestSignature>>,
}
impl TestBody {
    pub(crate) fn new(
        name: Name,
        child_name_factory: NameFactory,
        parent_code: Vec<TokenTree>,
    ) -> Self {
        Self {
            state:TestBodyState::Code,
            name,
            child_name_factory,
            code: parent_code,
            children: Vec::new(),
            partial_child: None,
        }
    }

    pub(crate) fn accept_token(&mut self, tok: &TokenTree) -> CompileResult<bool> {
        match self.state {
            TestBodyState::Code => match tok {
                TokenTree::Punct(punct) if punct.as_char() == '$' => {
                    self.partial_child = Some(Box::new(TestSignature::new(
                        tok.span(),
                        self.child_name_factory.clone(),
                        self.code.clone(),
                    )));
                    self.state = TestBodyState::ChildTest;
                    Ok(true)
                }
                _ => {
                    self.code.push(tok.clone());
                    Ok(true)
                }
            },

            TestBodyState::ChildTest => {
                if self.partial_child.as_mut().unwrap().accept_token(tok)? {
                    Ok(true)
                } else {
                    // test rejected the input without error which means that
                    // the test is now complete.
                    self.children.push(*self.partial_child.take().unwrap());
                    self.state = TestBodyState::Code;

                    // back to the start looking for more tests
                    self.accept_token(tok)
                }
            }
        }
    }

    pub(crate) fn generate_tokens(&mut self) -> CompileResult<Vec<TokenTree>> {
        if self.partial_child.is_some() {
            self.children.push(*self.partial_child.take().unwrap());
        }

        if self.children.is_empty() {
            Ok(vec![
                punct('#'),
                bracketed([ident("test", *self.name.span())]),
                ident("fn", *self.name.span()),
                ident(self.name.function_name()?.as_str(), *self.name.span()),
                parenthesised([]),
                braced(std::mem::take(&mut self.code)),
            ])
        } else {
            let mut child_tokens = Vec::new();
            for test in &mut self.children {
                let mut tokens = test.generate_tokens()?;
                child_tokens.append(&mut tokens);
            }

            Ok(child_tokens)
        }
    }
}
