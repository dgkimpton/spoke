pub(crate) use proc_macro2::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream as Stream, TokenTree,
};

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
pub(crate) fn braced_stream(stream: Stream) -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Brace, stream))
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
