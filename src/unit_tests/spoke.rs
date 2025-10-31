#[cfg(test)]
mod tests {
    use crate::spoke::generate_tests as generate;

    #[test]
    fn first_test() {
        let input = proc_macro2::TokenStream::new();
        let expected = proc_macro2::TokenStream::new();
        let result = generate(input);
        assert_eq!(
            expected.to_string(),
            result.to_string()
        )
    }
}
