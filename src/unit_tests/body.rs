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
    fn keeps_basic_code() {
        parse_valid(Input(
            r##"
                let x = 5;
                let y = 5;
                assert_eq!(x,y);
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
            # [test] fn a_test () { 
                let x = 5;
                let y = 5;
                assert_eq!(x,y);
            }
            "##,
        ));
    }

    #[test]
    fn nested_tests_are_name_sequentially() {
        parse_valid(Input(
            r##"
                $"inner spec 1"{
                }
                $"inner 2"{
                }
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
            #[test] 
            fn a_test_inner_spec_1() {
            }

            #[test] 
            fn a_test_inner_2() {
            }
            "##,
        ));
    }

    #[test]
    fn nested_tests_keep_their_respective_bodies() {
        parse_valid(Input(
            r##"
                $"inner spec 1"{
                    let x = 5;
                    let y = 5;
                    assert_eq!(x,y);
                }
                $"inner 2"{
                    let p = 6;
                    let q = 8;
                    assert_eq!(p, q);
                }
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                #[test] 
                fn a_test_inner_spec_1() {
                    let x = 5;
                    let y = 5;
                    assert_eq!(x,y);
                }

                #[test] 
                fn a_test_inner_2() {
                    let p = 6;
                    let q = 8;
                    assert_eq!(p, q);
                }
            "##,
        ));
    }

    #[test]
    fn nested_tests_associates_outer_code_correctly() {
        parse_valid(Input(
            r##"
                let x = 5;
                $"inner spec 1"{
                    let y = 5;
                    assert_eq!(x,y);
                }

                let p = 6;
                $"inner 2"{
                    let q = 8;
                    assert_eq!(p, q);
                }
                let n = 7;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                #[test] 
                fn a_test_inner_spec_1() {
                    let x = 5;
                    let y = 5;
                    assert_eq!(x,y);
                }

                #[test] 
                fn a_test_inner_2() {
                    let x = 5;
                    let p = 6;
                    let q = 8;
                    assert_eq!(p, q);
                }
            "##,
        ));
    }

    #[test]
    fn deeply_nested_tests_work() {
        parse_valid(Input(
            r##"
            $"level" {
                let x = 5;
                $"inner spec 1"{
                    let y = 5;
                    $"2nd"{
                        $"3rda" {
                            assert_eq!(x,y);
                        }
                        $"3rdb" {
                            assert_eq!(x*2,y*3);
                        }
                    }
                }

                let p = 6;
                $"inner 2"{
                    let q = 8;
                    assert_eq!(p, q);
                    $"2nd"{
                        $"3rd" {
                            assert_eq!(q*2,y*2);
                        }
                    }
                }
                let n = 7;
                $"simple" {
                    assert_eq!(n,7);
                }
            }
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                #[test] 
                fn a_test_level_inner_spec_1_2nd_3rda() {
                    let x = 5;
                    let y = 5;
                    assert_eq!(x,y);
                }
                #[test] 
                fn a_test_level_inner_spec_1_2nd_3rdb() {
                    let x = 5;
                    let y = 5;
                    assert_eq!(x*2,y*3);
                }

                #[test] 
                fn a_test_level_inner_2_2nd_3rd() {
                    let x = 5;
                    let p = 6;
                    let q = 8;
                    assert_eq!(p, q);
                    assert_eq!(q*2,y*2);
                }

                #[test]
                fn a_test_level_simple() {
                    let x = 5;
                    let p = 6;
                    let n = 7;
                    assert_eq!(n,7);
                }
            "##,
        ));
    }

    #[test]
    fn running_out_of_input_in_a_test_is_an_error() {
        parse_valid(Input(
            r##"
                $"test"
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("reached end of group input before finding the test body for named test");
            "##,
        ));
    }

    #[test]
    fn an_identifier_is_not_a_valid_test_name() {
        parse_valid(Input(
            r##"
                $test
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid test name following the dollars, but but found test");
                compile_error!("reached end of group input before reaching the end of the test definition");
            "##,
        ));
    }

    #[test]
    fn an_symbol_is_not_a_valid_test_name() {
        parse_valid(Input(
            r##"
                $#
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid test name following the dollars, but but found #");
                compile_error!("reached end of group input before reaching the end of the test definition");
            "##,
        ));
    }

    #[test]
    fn an_group_is_not_a_valid_test_name_is_assumed_to_be_a_missing_name() {
        parse_valid(Input(
            r##"
                ${}
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid test name following the dollars, but but found { }");
                # [test] fn a_test_missing_name () { }
            "##,
        ));
    }

    #[test]
    fn an_semicolon_is_not_a_valid_test_name() {
        parse_valid(Input(
            r##"
                $;
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("expected a valid test name following the dollars, but found a truncated assertion - found `;` before any code was provided");
            "##,
        ));
    }
    #[test]
    fn running_out_of_input_before_the_name_is_an_error() {
        parse_valid(Input(
            r##"
                $
            "##,
        ))
        .matches_inside::<SuiteStructure>(Expected(
            r##"
                compile_error!("reached end of group input before reaching the end of the test definition");
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
                Name::new(&Span::call_site(), "a_test"),
                group,
                &mut output,
            ),
            _ => panic!("body parsers can only parse groups"),
        };

        output.generate_output()
    }
}
