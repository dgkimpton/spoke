pub(crate) use proc_macro2::TokenTree;

pub(crate) trait TokenGenerator {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>);
}

pub(crate) trait IntoVecTokens {
    fn into_vec(&mut self) -> Vec<TokenTree>;
}

impl<T: TokenGenerator> IntoVecTokens for T {
    fn into_vec(&mut self) -> Vec<TokenTree> {
        let mut content = Vec::<TokenTree>::new();
        self.generate_tokens(&mut content);
        content
    }
}
