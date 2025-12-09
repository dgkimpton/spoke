use crate::{parse, parser::*};

pub(crate) fn generate_tests(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut suite = SuiteGenerator::new();

    if !input.is_empty() {
        let mut current_rule = ParseRule::Suite(parse::Suite());
        for token in input.into_iter() {
            current_rule = current_rule.accept_token(token, &mut suite);
        }

        current_rule.end_of_stream(&mut suite);
    }

    suite.generate_output()
}
