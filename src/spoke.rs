use crate::{suite, token_helpers};

pub(crate) fn generate_tests(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    
    token_helpers::in_stream(
        match suite::process(input) {
            Ok(out) => out,
            Err(e) => e.generate_tokens(),
        },
    )
}
