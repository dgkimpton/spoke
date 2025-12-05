use crate::{code_block::CodeBlock, name::*, parse, parser::*};

pub(crate) struct TransientAssertAnchor {
    parent: parse::Body,
    anchor: Span,
    name: Name,
    left_code: CodeBlock,
}
impl TransientAssertAnchor {
    pub(crate) fn new(
        parent: parse::Body,
        name: Name,
        left_code: CodeBlock,
        location: &impl SpanSource,
    ) -> Self {
        Self {
            parent,
            name,
            left_code,
            anchor: location.span(),
        }
    }
}

impl Parser for TransientAssertAnchor {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Ident(ident) => match parse::dollars_ident_match(ident.to_string()) {
                Ok(assert) => self.route_assert(&ident, assert, target),
                Err(rejected) => match parse::dollars_ident_match(rejected.to_ascii_lowercase()) {
                    Ok(assert) => {
                        expected_assert(
                            &ident,
                            format!(
                                "found an incorrectly cased match `{}` - asserts are all lowercase",
                                rejected
                            ),
                            target,
                        );
                        self.route_assert(&ident, assert, target)
                    }
                    Err(invalid) => {
                        expected_assert(&ident, format!("found `{}`", invalid), target);
                        parse::TransientAssertError::new(self.parent).consumed_token()
                    }
                },
            },
            TokenTree::Punct(punct) if punct.as_char() == ';' => {
                // seems the test is malformed, look for more tests
                expected_assert(
                    &punct,
                    "expected an assertion after the name, but found `;`",
                    target,
                );
                ParseRule::Body(self.parent)
            }
            other => {
                let found = other.to_string();
                expected_assert(&other, format!("found `{}`", found), target);

                parse::TransientAssertError::new(self.parent).forward_token(other, target)
            }
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &self.anchor,
            "RULE::TransientAssertAnchor: reached end of group input before reaching the end of the assertion definition",
        );
        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.anchor,
            "RULE::TransientAssertAnchor: reached end of input before reaching the end of the assertion definition",
        );
    }
}

fn expected_assert(
    location: &impl SpanSource,
    err: impl Into<String>,
    target: &mut SuiteGenerator,
) {
    target.push_new_error(
        location,
        format!(
            "RULE::TransientAssertAnchor: expected a valid assertion type following the dollars, but {}",
            err.into()
        ),
    );
}

impl TransientAssertAnchor {
    fn route_assert(
        self,
        location: &impl SpanSource,
        assert_type: parse::Dollars,
        _target: &mut SuiteGenerator,
    ) -> ParseRule {
        match assert_type {
            parse::Dollars::AssertEq => parse::AssertEq::new(
                self.parent,
                self.name,
                &self.anchor,
                self.left_code,
                location,
            )
            .consumed_token(),
        }
    }
}

pub(crate) struct TransientAssertError {
    parent: parse::Body,
}

impl TransientAssertError {
    fn new(parent: parse::Body) -> Self {
        Self { parent }
    }
}

impl Parser for TransientAssertError {
    fn accept_token(self, token: TokenTree, _: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == ';' => {
                // seems the test is malformed, look for more tests
                ParseRule::Body(self.parent)
            }

            _ => self.consumed_token(),
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &Span::call_site(),
            "RULE::TransientAssertError: reached end of group input before reaching the end of the assertion definition",
        );

        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &Span::call_site(),
            "RULE::TransientAssertError: reached end of input before reaching the end of the assertion definition",
        );
    }
}
