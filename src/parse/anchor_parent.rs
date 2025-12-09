use crate::{name::*, parse, parser::*};

pub(crate) enum AnchorParent {
    Suite(parse::Suite),
    Body(Box<parse::Body>),
}
impl AnchorParent {
    pub(crate) fn from_body(body:parse::Body) -> Self {
        Self::Body(Box::new(body))
    }
    pub(crate) fn from_suite(suite:parse::Suite) -> Self {
        Self::Suite(suite)
    }
    pub(crate) fn continuation(self) -> ParseRule {
        match self {
            AnchorParent::Suite(suite) => ParseRule::Suite(suite),
            AnchorParent::Body(body) => ParseRule::Body(*body),
        }
    }
}

impl Nameable for AnchorParent {
    fn collect_name_parts<'a>(&'a self, compound: CompoundName<'a>) -> CompoundName<'a> {
        match &self {
            AnchorParent::Suite(_) => compound,
            AnchorParent::Body(body) => body.collect_name_parts(compound),
        }
    }
}

impl Populator for AnchorParent {
    fn populate_test(&self, test: TestCase) -> TestCase {
        match &self {
            AnchorParent::Suite(_) => test,
            AnchorParent::Body(body) => body.populate_test(test),
        }
    }
}
