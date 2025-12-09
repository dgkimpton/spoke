#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    #[allow(unused_imports)]
    use super::*;

    use crate::{name::*, parse, parser::*, unit_tests::testing_helpers::*};

    struct SuiteStructure();
    impl SurroundingString for SuiteStructure {
        fn surround(input: &str) -> String {
            format!(
                "#[cfg(test)] #[allow(unused_mut)] #[allow(unused_variables)]  mod spoketest {{ {} }}",
                input
            )
        }
    }


    #[test]
    fn can_place_an_assertion_instead_of_a_body() {
        parse_valid(Input(
            r##"
                $"test" true $eq false;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                #[test] 
                fn inner_test() {
                    assert_eq!(true, false);
                }
            "##,
        ));
    }

    #[test]
    fn can_place_a_negative_assertion_instead_of_a_body() {
        parse_valid(Input(
            r##"
                $"test" true $ne false;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                #[test] 
                fn inner_test() {
                    assert_ne!(true, false);
                }
            "##,
        ));
    }

    #[test]
    fn half_an_assertion_is_an_error_left() {
        parse_valid(Input(
            r##"
                $"test" $ne false;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("no code found for the left side of the equality assertion");
            "##,
        ));
    }

    #[test]
    fn half_an_assertion_is_an_error_right() {
        parse_valid(Input(
            r##"
                $"test" true $ne;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
               compile_error!("no code found for the right hand side of the equality assertion");
            "##,
        ));
    }

    #[test]
    fn half_an_assertion_due_to_truncated_input_is_an_error_right() {
        parse_valid(Input(
            r##"
                $"test" true $ne
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
               compile_error!("reached end of group input before reaching the end of the equality assertion definition");
            "##,
        ));
    }

    #[test]
    fn an_empty_assertion_produces_a_compile_error() {
        parse_valid(Input(
            r##"
                $"test";
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected an assertion or test body after the name, but found `;`");
            "##,
        ));
    }

    #[test]
    fn running_out_of_input_in_an_assertion_is_an_error() {
        parse_valid(Input(
            r##"
                $"test" true
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("reached end of group input before finding details of the named assertion. Missing ; ?");
            "##,
        ));
    }

    #[test]
    fn an_assertion_with_no_modifier_is_an_assert() {
        parse_valid(Input(
            r##"
                $"test" true;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"#[test] 
                fn inner_test() {
                    assert!(true);
                }
            "##,
        ));
    }

    #[test]
    fn an_assertion_with_no_modifier_can_take_complex_code() {
        parse_valid(Input(
            r##"
                $"test" 7 < {let x = 5; x *2 } ;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"#[test] 
                fn inner_test() {
                    assert!(7 < {let x = 5; x *2});
                }
            "##,
        ));
    }

    #[test]
    fn an_assertion_with_no_modifier_can_a_body_inside_parens() {
        parse_valid(Input(
            r##"
                $"test" ({let x = 5; x *2 } > 7) ;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"#[test] 
                fn inner_test() {
                    assert!(({let x = 5; x *2 } > 7));
                }
            "##,
        ));
    }

    #[test]
    fn an_uknown_assertion_type_is_an_error() {
        parse_valid(Input(
            r##"
                $"test" true $plop ;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid assertion type [eq,ne] following the dollars, but found `plop`");
            "##,
        ));
    }

    #[test]
    fn case_sensitivity_matters_for_assertion_names() {
        parse_valid(Input(
            r##"
                $"test" true $EQ false;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid assertion type [eq,ne] following the dollars, but found an incorrectly cased match `EQ` - asserts are all lowercase");
                #[test]
                fn inner_test () {
                    assert_eq!(true, false); 
                }
            "##,
        ));
    }


    #[test]
    fn cannot_have_an_assertion_in_a_body() {
        parse_valid(Input(
            r##"
                $"test" { $ne }
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
            compile_error!("expected a valid test name following the dollars, but found an assertion 'ne' which isn't allowed inside the braced body of a test");
            compile_error!("reached end of group input before reaching the end of the test definition");
            "##,
        ));
    }

    #[test]
    fn cannot_have_an_assertion_in_a_body_with_an_badly_cased_name() {
        parse_valid(Input(
            r##"
                $"test" { $NE }
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
            compile_error!("expected a valid test name following the dollars, but found a badly formatted assertion 'ne' which isn't allowed inside the braced body of a test and should be lowercase");
            compile_error!("reached end of group input before reaching the end of the test definition");
            "##,
        ));
    }

    #[test]
    fn a_literal_is_not_an_assertion() {
        parse_valid(Input(
            r##"
                $"test" true $"lit" ;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid assertion type [eq,ne] following the dollars, but found `\"lit\"` which looks like a test defintion. Tests cannot be nested inside asserts.");
                compile_error!("expected an assertion or test body after the name, but found `;`"); 
            "##,
        ));
    }

    #[test]
    fn a_semi_colon_isnt_a_valid_assert_name() {
        parse_valid(Input(
            r##"
                $"test" true $;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid assertion type [eq,ne] following the dollars, but found `;`");
            "##,
        ));
    }

    #[test]
    fn a_symbol_isnt_a_valid_assert_name() {
        parse_valid(Input(
            r##"
                $"test" true $#;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid assertion type [eq,ne] following the dollars, but found `#`");
            "##,
        ));
    }

    #[test]
    fn end_of_input_isnt_a_valid_assert_name() {
        parse_valid(Input(
            r##"
                $"test" true $
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("reached end of group input before reaching the end of the assertion definition");
            "##,
        ));
    }

    #[test]
    fn end_of_input_isnt_a_valid_assert_error_recovery() {
        parse_valid(Input(
            r##"
                $"test" true $#
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid assertion type [eq,ne] following the dollars, but found `#`");
                compile_error!("reached end of group input before reaching the end of the assertion definition");
            "##,
        ));
    }

    fn parse_valid(input: Input) -> proc_macro2::TokenStream {
        let mut output = SuiteGenerator::new();

        let tok = Input(format!("{{ {} }}", input.0).as_str())
            .stream()
            .into_iter()
            .next()
            .expect("there should be valid input");

        match tok {
            proc_macro2::TokenTree::Group(group) => parse::Body::generate_body_in_suite(
                parse::Suite(),
                Name::new(&Span::call_site(), "inner"),
                group,
                &mut output,
            ),
            _ => panic!("body parsers can only parse groups"),
        };

        output.generate_output()
    }
}
