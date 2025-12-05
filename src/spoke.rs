use crate::{parse, parser::*};

pub(crate) fn generate_tests(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut suite = SuiteGenerator::new();

    if !input.is_empty() {
        input.process_into(ParseRule::Suite(parse::Suite()), &mut suite);
    }

    suite.generate_output()
}
