use crate::{name::*, parse, parser::*};

pub(crate) struct TransientSuiteAnchor {
    parent: parse::Suite,
    anchor: Span,
}
impl TransientSuiteAnchor {
    pub(crate) fn new(parent: parse::Suite, location: &impl SpanSource) -> Self {
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
                "expected a valid test case name in quotes following the dollars, but {}",
                err.into()
            ),
        );

        parse::TransientSuiteNamingError::new(self.parent, target.make_missing_name(&token))
            .forward_token(token, target)
    }
}

impl Parser for TransientSuiteAnchor {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match parse::dollars_match(&token){
            parse::MatchResult::Name(name) => parse::TransientSuiteNamed::new(self.parent, name).consumed_token(),
            parse::MatchResult::InvalidName(error) => self.expected_name(token, error, target),
            parse::MatchResult::ValidDollars(dollars) => self.expected_name(
                    token,
                    format!("found an assertion '{}' which isn't allowed as a top level test", dollars),
                    target,
                ),
            parse::MatchResult::InvalidDollars(dollars) => self.expected_name(
                    token,
                    format!("found a badly formatted assertion '{}' which isn't allowed as a top level test and should be lowercase", dollars),
                    target,
                ),
            parse::MatchResult::OtherInvalid(found) => self.expected_name(token, format!("found `{}`", found), target), 
            parse::MatchResult::SemiColon => self.expected_name(
                    token,
                    "found `;` before any body was provided",
                    target
                ),
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        // Note: I can't think of any reason why this would ever happen but the type system
        // isn't quite flexible enough to let me express that
        target.push_new_error(
            &self.anchor,
            "reached end of group input before reaching the end of the test definition",
        );
        ParseRule::Suite(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.anchor,
            "reached end of input before reaching the end of the test definition",
        );
    }
}

pub(crate) struct TransientSuiteNamed {
    parent: parse::Suite,
    name: Name,
}
impl TransientSuiteNamed {
    fn new(parent: parse::Suite, name: Name) -> Self {
        Self { parent, name }
    }
}
impl Parser for TransientSuiteNamed {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                parse::Body::generate_body_in_suite(self.parent, self.name, group, target)
            }

            token_tree => {
                parse::Assert::from_suite(self.parent, self.name)
                    .forward_token(token_tree, target)
            }
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &self.name.span(),
            "reached end of group input before finding the test body for named test",
        );
        ParseRule::Suite(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &self.name.span(),
            "reached end of input before finding the test body for named test",
        );
    }
}

pub(crate) struct TransientSuiteNamingError {
    parent: parse::Suite,
    name: Name,
}

impl TransientSuiteNamingError {
    fn new(parent: parse::Suite, name: Name) -> Self {
        Self { parent, name }
    }
}

impl Parser for TransientSuiteNamingError {
    fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                // assume this is probably the body of the test

                parse::Body::generate_body_in_suite(self.parent, self.name, group, target)
            }

            TokenTree::Punct(punct) if punct.as_char() == ';' => {
                // seems the test is malformed, look for more tests
                ParseRule::Suite(self.parent)
            }

            _ => self.consumed_token(),
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        target.push_new_error(
            &Span::call_site(),
            "reached end of group input before reaching the end of the test definition",
        );

        ParseRule::Suite(self.parent)
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        target.push_new_error(
            &Span::call_site(),
            "reached end of input before reaching the end of the test definition",
        );
    }
}
