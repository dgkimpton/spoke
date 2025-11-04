use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenTree};

pub(crate) trait SpanSource {
    fn span(&self) -> Span;
}

macro_rules! impl_span_source_for {
    ($($t:ty),+ $(,)?) => {$(
        impl SpanSource for $t {
            #[inline]
            fn span(&self) -> Span { self.span() }
        }
    )+};
}

impl SpanSource for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl_span_source_for!(Ident, Literal, Punct, Group, TokenTree,);
