#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use crate::spoke::generate_tests as generate;

    #[test]
    fn empty_input_produces_empty_output() {
        let input = proc_macro2::TokenStream::new();
        let expected = proc_macro2::TokenStream::new();
        let result = generate(input);
        assert_eq!(
            expected.to_string(),
            result.to_string()
        )
    }
    
    #[test]
    fn a_broken_input_produces_an_errpr() {
        let input : TokenStream = "$$".parse().expect("tokens");
        let expected  : TokenStream= "compile_error ! (\"expected name of the testcase :: `expected a string literal (e.g. \\\"Ferris\\\"), but found a punctuation character`\") ;".parse().expect("tokens");
        let result = generate(input);
        assert_eq!(
            expected.to_string(),
            result.to_string()
        )
    }
}
