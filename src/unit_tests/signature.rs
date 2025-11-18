#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::{consumer::TokenStreamExt, name::*, signature::*, unit_tests::testing_helpers::*};

    #[test]
    fn input_output_matching_works_for_empty() {
        Input("").stream().matches(Expected(""));
    }

    #[test]
    fn input_output_matching_works_for_rust_test() {
        Input(
            r##"
                #[test] fn a_test() {assert_eq!(true, true)}
            "##,
        )
        .stream()
        .matches(Expected(
            r##"
                #[test] fn a_test() {assert_eq!(true, true)}
            "##,
        ));
    }

    #[test]
    fn test_signature_generation_fails_if_generated_with_no_tokens() {
        Signature::new(&proc_macro2::Span::call_site(), NameFactory::new())
            .generate()
            .matches(Expected(
                r##"
                    compile_error!("expected a test name in quotes");
                "##,
            ));
    }

    // #[test]
    // fn test_signature_generation_fails_if_generated_with_wrong_tokens_where_name_expected() {
    //     let mut test = parse_valid(Input(""));

    //     let sp = Span::call_site();
    //     assert_eq!(
    //         CompileError::err(
    //             &sp,
    //             "in SUITE  :: error parsing tests : expected name of the testcase :: `expected a string literal (e.g. \"Ferris\"), but found an identifier` :: test"
    //         ),
    //         test.accept_token(&ident("test", sp))
    //     );
    // }

    #[test]
    fn test_signature_generation_fails_if_generated_with_no_body_and_no_assert() {
        parse_valid(Input(
            r##"
                "test"
            "##,
        ))
        .generate()
        .matches(Expected(
            r##"
                compile_error!("test case specified without a body at `test`. expected a test body in braces or a valid assertion");
            "##,
        ));
    }

    #[test]
    fn can_parse_simple_test_signature_and_generate_a_simple_test() {
        parse_valid(Input(
            r##"
                "test"{}
            "##,
        ))
        .generate()
        .matches(Expected(
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
        .generate()
        .matches(Expected(
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

    fn parse_valid(input: Input) -> Signature {
        let test = Signature::new(&proc_macro2::Span::call_site(), NameFactory::new());
        let (result, _error) = input.stream().process_into(test);
        result
    }
}
