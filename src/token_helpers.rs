pub(crate) use proc_macro2::*;

pub(crate) trait IterableTokens: IntoIterator<Item = TokenTree> {}
impl<T: IntoIterator<Item = TokenTree>> IterableTokens for T {}

pub(crate) fn punct(c: char, sp: Span) -> TokenTree {
    let mut punct = TokenTree::Punct(Punct::new(c, Spacing::Alone));
    punct.set_span(sp);
    punct
}

pub(crate) fn ident(id: &str, sp: Span) -> TokenTree {
    TokenTree::Ident(Ident::new(id, sp))
}

pub(crate) fn lit_string(s: &str, sp: Span) -> TokenTree {
    let mut lit = TokenTree::Literal(Literal::string(s));
    lit.set_span(sp);
    lit
}

pub(crate) fn braced(toks: impl IterableTokens, sp: Span) -> TokenTree {
    group(Delimiter::Brace, toks, sp)
}
pub(crate) fn braced_stream(stream: TokenStream) -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Brace, stream))
}
pub(crate) fn parenthesised(toks: impl IterableTokens, sp: Span) -> TokenTree {
    group(Delimiter::Parenthesis, toks, sp)
}
pub(crate) fn bracketed(toks: impl IterableTokens, sp: Span) -> TokenTree {
    group(Delimiter::Bracket, toks, sp)
}

fn group(delim: Delimiter, toks: impl IterableTokens, sp: Span) -> TokenTree {
    let mut stream = TokenStream::new();
    stream.extend(toks);
    let mut grp = TokenTree::Group(Group::new(delim, stream));
    grp.set_span(sp);
    grp
}
