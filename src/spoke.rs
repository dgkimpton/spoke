
pub(crate) fn generate_tests(_input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}

#[cfg(test)]
mod tests {

    #[test]
    fn first_test() {
        assert_eq!(super::generate_tests(proc_macro2::TokenStream::new()).to_string(), "".to_string())
    }
}