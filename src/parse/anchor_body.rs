use crate::{name::*, parse, parser::*};

pub(crate) struct TransientBodyAnchor {
    parent: parse::Body,
    anchor: Span,
}
impl TransientBodyAnchor {
    pub(crate) fn new(parent: parse::Body, location: &impl SpanSource) -> Self {
        Self {
            parent,
            anchor: location.span(),
        }
    }

    fn expected_name(
        self,
        token: TokenTree,
        err: impl Into<String>,
        target: &mut SuiteGenerator,
    ) -> ParseRule {
        target.push_new_error(
            &token,
            format!(
                "expected a valid test name following the dollars, but {}",
                err.into()
            ),
        );

        parse::TransientBodyNamingError::new(self.parent, target.make_missing_name(&token))
            .forward_token(token, target)
    }
}

impl Parser for TransientBodyAnchor {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match parse::dollars_match(&token){
            parse::MatchResult::Name(name) => parse::TransientBodyNamed::new(self.parent, name).consumed_token(),
            parse::MatchResult::InvalidName(error) => self.expected_name(token, error, target),
            parse::MatchResult::ValidDollars(dollars) => self.expected_name(
                    token,
                    format!("found an assertion '{}' which isn't allowed inside the braced body of a test", dollars),
                    target,
                ),
            parse::MatchResult::InvalidDollars(dollars) => self.expected_name(
                    token,
                    format!("found a badly formatted assertion '{}' which isn't  allowed inside the braced body of a test and should be lowercase", dollars),
                    target
                ),
            parse::MatchResult::OtherInvalid(found) => self.expected_name(token, format!("but found {}", found), target),
            parse::MatchResult::SemiColon => self.expected_name(
                    token,
                    "truncated assertion - found `;` before any code was provided",
                    target
                ),
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &self.anchor,
            "reached end of group input before reaching the end of the test definition",
        );
        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.anchor,
            "reached end of input before reaching the end of the test definition",
        );
    }
}

pub(crate) struct TransientBodyNamed {
    parent: parse::Body,
    name: Name,
}

impl TransientBodyNamed {
    fn new(parent: parse::Body, name: Name) -> Self {
        Self { parent, name }
    }
}

impl Parser for TransientBodyNamed {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                parse::Body::from_body(self.parent, self.name, group, target)
            }

            token_tree => {
                parse::Assert::new(self.parent, self.name)
                    .forward_token(token_tree, target)
            }
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &self.name.span(),
            "reached end of group input before finding the test body for named test",
        );
        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.name.span(),
            "reached end of input before finding the test body for named test",
        );
    }
}

pub(crate) struct TransientBodyNamingError {
    parent: parse::Body,
    name: Name,
}

impl TransientBodyNamingError {
    fn new(parent: parse::Body, name: Name) -> Self {
        Self { parent, name }
    }
}

impl Parser for TransientBodyNamingError {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                // assume this is probably the body of the test

                parse::Body::from_body(self.parent, self.name, group, target)
            }

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
            "reached end of group input before reaching the end of the test definition",
        );

        ParseRule::Body(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &Span::call_site(),
            "reached end of input before reaching the end of the test definition",
        );
    }
}
