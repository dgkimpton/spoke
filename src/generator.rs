use crate::{code_block::*, name::Name, parser::*, token_helpers::*};
use std::mem::take;

pub(crate) struct SuiteGenerator {
    errors: Vec<CompilationError>,
    preamble: CodeBlock,
    tests: Vec<TestCase>,
    error_counter: usize,
}

pub(crate) struct CompilationError {
    span: proc_macro2::Span,
    msg: String,
}

pub(crate) struct TestCase {
    name: String,
    anchor: Span,
    code: Vec<TokenTree>,
}

impl CompilationError {
    pub(crate) fn new(msg: impl Into<String>, span: &impl SpanSource) -> Self {
        Self {
            span: span.span(),
            msg: msg.into(),
        }
    }
}

impl SuiteGenerator {
    pub(crate) fn new() -> Self {
        Self {
            errors: Vec::new(),
            preamble: CodeBlock::new(),
            tests: Vec::new(),
            error_counter: 0,
        }
    }

    pub(crate) fn push_preamble(&mut self, token: TokenTree) {
        self.preamble.push(token);
    }

    pub(crate) fn push_test(&mut self, test: TestCase) {
        self.tests.push(test);
    }

    pub(crate) fn push_error(&mut self, error: CompilationError) {
        self.errors.push(error);
    }

    pub(crate) fn push_new_error(&mut self, location: &impl SpanSource, msg: impl Into<String>) {
        self.push_error(CompilationError::new(msg, location));
    }

    pub(crate) fn make_missing_name(&mut self, location: &impl SpanSource) -> crate::name::Name {
        self.error_counter += 1;
        Name::missing(location, self.error_counter)
    }
}

impl TestCase {
    pub(crate) fn push_code(&mut self, token: impl IterableTokens) {
        self.code.extend(token)
    }

    pub(crate) fn new((name, anchor): (String, Span)) -> Self {
        Self {
            name,
            anchor,
            code: CodeBlock::new(),
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////

impl SuiteGenerator {
    fn is_empty(&self) -> bool {
        self.errors.is_empty() && self.preamble.is_empty() && self.tests.is_empty()
    }

    pub(crate) fn generate_output(self) -> TokenStream {
        let call_site = Span::call_site();
        let mut output = TokenStream::new();

        if self.is_empty() {
            return output;
        }

        output.extend([
            punct('#', call_site),
            bracketed(
                [
                    ident("cfg", call_site),
                    parenthesised([ident("test", call_site)], call_site),
                ],
                call_site,
            ),
            punct('#', call_site),
            bracketed(
                [
                    ident("allow", call_site),
                    parenthesised([ident("unused_mut", call_site)], call_site),
                ],
                call_site,
            ),
            punct('#', call_site),
            bracketed(
                [
                    ident("allow", call_site),
                    parenthesised([ident("unused_variables", call_site)], call_site),
                ],
                call_site,
            ),
            ident("mod", call_site),
            ident("spoketest", call_site),
            braced_stream(self.generate_suite()),
        ]);

        output
    }

    fn generate_suite(mut self) -> TokenStream {
        let mut output = TokenStream::new();
        for error in self.errors {
            error.generate_into(&mut output)
        }

        output.extend(take(&mut self.preamble).into_iter());

        for mut test in self.tests {
            test.generate_into(&mut output)
        }

        output
    }
}

impl TestCase {
    fn generate_into(&mut self, output: &mut TokenStream) {
        output.extend([
            punct('#', self.anchor),
            bracketed([ident("test", self.anchor)], self.anchor),
            ident("fn", self.anchor),
            ident(self.name.as_str(), self.anchor),
            parenthesised([], self.anchor),
            braced(take(&mut self.code).into_iter(), self.anchor),
        ])
    }
}

impl CompilationError {
    fn generate_into(&self, output: &mut TokenStream) {
        let span = self.span;
        output.extend([
            ident("compile_error", span),
            punct('!', span),
            parenthesised([lit_string(&self.msg, span)], span),
            punct(';', span),
        ]);
    }
}
