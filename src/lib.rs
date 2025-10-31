mod spoke;

/// Placeholder crate reserved for future development.

#[proc_macro]
pub fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::spoke::generate_tests(proc_macro2::TokenStream::from(input)).into()
}
