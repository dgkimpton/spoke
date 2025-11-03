use crate::span_source::*;

pub(crate) type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug)]
pub(crate) struct CompileError {
    span: proc_macro2::Span,
    msg: String,
}

impl PartialEq for CompileError {
    fn eq(&self, other: &Self) -> bool {
        self.span.start() == other.span.start()
            && self.span.end() == other.span.end()
            && self.msg == other.msg
    }
}

impl CompileError {
    pub(crate) fn new(span: proc_macro2::Span, msg: String) -> Self {
        Self { span, msg }
    }

    pub(crate) fn err<TSuccess>(
        span: &impl SpanSource,
        msg: impl Into<String>,
    ) -> Result<TSuccess, Self> {
        Err(Self::new(span.span(), msg.into()))
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing tests : {}", self.msg)
    }
}
