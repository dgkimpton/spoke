#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    use proc_macro2::Span;

    use crate::{error::*, name::*, signature::*, unit_tests::testing_helpers::*, token_helpers::*};

    #[test]
    fn input_output_matching_works_for_empty() {
        Input("").stream().matches_ok(Expected(""));
    }

    #[test]
    fn input_output_matching_works_for_rust_test() {
        Input(
            r##"
                #[test] fn a_test() {assert_eq!(true, true)}
            "##,
        )
        .stream()
        .matches_ok(Expected(
            r##"
                #[test] fn a_test() {assert_eq!(true, true)}
            "##,
        ));
    }

    #[test]
    fn test_signature_generation_fails_if_generated_with_no_tokens() {
        TestSignature::new(proc_macro2::Span::call_site(), NameFactory::new(), [])
            .generate_tokens()
            .matches_failure(Expected(
                r##"
                    compile_error!("expected a test name in quotes");
                "##,
            ));
    }

    #[test]
    fn test_signature_generation_fails_if_generated_with_wrong_tokens_where_name_expected() {
        let mut test = parse_valid(Input(""));

        let sp = Span::call_site();
        assert_eq!(
            CompileError::err(
                &sp,
                "expected name of the testcase :: `expected a string literal (e.g. \"Ferris\"), but found an identifier`"
            ),
            test.accept_token(&ident("test", sp))
        );
    }

    #[test]
    fn test_signature_generation_fails_if_generated_with_no_body() {
        parse_valid(Input(
            r##"
                "test"
            "##,
        ))
        .generate_tokens()
        .matches_failure(Expected(
            r##"
                compile_error!("test case specified without a body at `test`. expected a test body in braces");
            "##,
        ));
    }

    #[test]
    fn test_signature_generation_fails_if_generated_with_wrong_tokens_where_body_expected() {
        let mut test = parse_valid(Input(
            r##"
        "test"
        "##,
        ));

        let sp = Span::call_site();
        assert_eq!(
            CompileError::err(
                &sp,
                "expected to find the body of the test in braces, but :: got `test`"
            ),
            test.accept_token(&ident("test", sp))
        );
    }

    #[test]
    fn can_parse_simple_test_signature_and_generate_a_simple_test() {
        parse_valid(Input(
            r##"
                "test"{}
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[test] 
                fn test() {}
            "##,
        ));
    }
    
    #[test]
    fn can_parse_test_with_included_code() {
        parse_valid(Input(
            r##"
                "test"{
                    let x = 5;
                    let y = 5;
                    assert_eq!(x,y);
                }
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[test] 
                fn test() {
                    let x = 5;
                    let y = 5;
                    assert_eq!(x,y);
                }
            "##,
        ));
    }

    fn parse_valid(input: Input) -> TestSignature {
        let mut test = TestSignature::new(proc_macro2::Span::call_site(), NameFactory::new(), []);
        for tok in input.stream().expect("valid token stream") {
            assert_eq!(Ok(true), test.accept_token(&tok));
        }
        test
    }
}
