pub(crate) use proc_macro2::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream as Stream, TokenTree,
};

use crate::error::CompileError;

pub(crate) trait IterableTokens: IntoIterator<Item = TokenTree> {}
impl<T: IntoIterator<Item = TokenTree>> IterableTokens for T {}

pub(crate) fn in_stream(toks: impl IterableTokens) -> Stream {
    let mut stream = Stream::new();
    stream.extend(toks);
    stream
}

pub(crate) fn punct(c: char) -> TokenTree {
    TokenTree::Punct(Punct::new(c, Spacing::Alone))
}

pub(crate) fn ident(id: &str, sp: Span) -> TokenTree {
    TokenTree::Ident(Ident::new(id, sp))
}

pub(crate) fn lit_string(s: String) -> TokenTree {
    TokenTree::Literal(Literal::string(s.as_str()))
}

pub(crate) fn braced(toks: impl IterableTokens) -> TokenTree {
    group(Delimiter::Brace, toks)
}
pub(crate) fn parenthesised(toks: impl IterableTokens) -> TokenTree {
    group(Delimiter::Parenthesis, toks)
}
pub(crate) fn bracketed(toks: impl IterableTokens) -> TokenTree {
    group(Delimiter::Bracket, toks)
}

fn group(delim: Delimiter, toks: impl IterableTokens) -> TokenTree {
    let mut stream = Stream::new();
    stream.extend(toks);
    TokenTree::Group(Group::new(delim, stream))
}

pub(crate) trait TokenTreeExtensions {
    fn expect_string_literal(&self, requirement: &str) -> Result<String, CompileError>;
}

impl TokenTreeExtensions for TokenTree {
    fn expect_string_literal(&self, requirement: &str) -> Result<String, CompileError> {
        match litrs::StringLit::try_from(self) {
            Ok(string_lit) => Ok(string_lit.value().to_string()),
            Err(e) => CompileError::err(self, format!("{requirement} :: `{e}`")),
        }
    }
}
