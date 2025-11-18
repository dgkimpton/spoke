use crate::{
    consumer::TokenStreamExt, generator::TokenGenerator, name::NameFactory, suite::Suite};

pub(crate) fn generate_tests(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut output = Vec::new();

    if !input.is_empty() {
        let (mut suite,mut error) = input.process_into(Suite::new(NameFactory::new()));

        suite.generate_tokens(&mut output);

        if let Some(error) = &mut error {
            error.generate_tokens(&mut output);
        }
    }

    crate::token_helpers::in_stream(output)
}
