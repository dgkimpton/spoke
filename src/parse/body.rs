use crate::{code_block::*, name::*, parse, parser::*};

pub(crate) struct Body {
    parent: parse::AnchorParent,
    name: Name,
    code: CodeBlock,
    has_children: bool,
}

impl Body {
    pub(crate) fn from_suite(
        parent: parse::Suite,
        name: Name,
        group: Group,
        target: &mut SuiteGenerator,
    ) -> ParseRule {
        Self::new(parse::AnchorParent::from_suite(parent), name, group, target)
    }

    pub(crate) fn new(
        parent: parse::AnchorParent,
        name: Name,
        group: Group,
        target: &mut SuiteGenerator,
    ) -> ParseRule {
        group.process_into(
            ParseRule::Body(Self {
                parent,
                name,
                code: CodeBlock::new(),
                has_children: false,
            }),
            target,
        )
    }

    fn generate_test(&self, target: &mut SuiteGenerator) {
        if !self.has_children {
            target.push_test(self.populate_test(TestCase::new(
                self.collect_name_parts(CompoundName::new()).function_name(),
            )));
        }
    }
}

impl Parser for Body {
    fn accept_token(mut self, token: TokenTree, _: &mut SuiteGenerator) -> ParseRule {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == '$' => {
                self.has_children = true;
                parse::TransientBodyAnchor::new(parse::AnchorParent::from_body(self), &punct).consumed_token()
            }

            other => {
                self.code.push(other);
                self.consumed_token()
            }
        }
    }

    fn end_of_group(self, target: &mut SuiteGenerator) -> ParseRule {
        self.generate_test(target);
        self.parent.continuation()
    }

    fn end_of_stream(self, target: &mut SuiteGenerator) {
        self.generate_test(target);
    }
}

impl Populator for Body {
    fn populate_test(&self, test: TestCase) -> TestCase {
        let mut test = self.parent.populate_test(test);
        test.push_code(self.code.clone());
        test
    }
}

impl Nameable for Body {
    fn collect_name_parts<'a>(&'a self, compound: CompoundName<'a>) -> CompoundName<'a> {
        self.parent
            .collect_name_parts(compound)
            .followed_by(&self.name)
    }
}
