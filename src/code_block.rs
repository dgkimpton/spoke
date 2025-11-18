use std::mem::take;

use proc_macro2::TokenTree;

#[derive(Default, Clone)]
pub(crate) struct CodeBlock {
    code: Vec<TokenTree>,
}
impl CodeBlock {
    pub(crate) fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub(crate) fn push(&mut self, value: TokenTree) {
        self.code.push(value);
    }

    pub(crate) fn take(&mut self) -> Vec<TokenTree> {
        take(&mut self.code)
    }
}
