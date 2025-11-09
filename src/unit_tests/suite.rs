#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    use crate::{name::*, suite::TestSuite, unit_tests::testing_helpers::*};

    #[test]
    fn an_empty_input_produces_no_output() {
        parse_valid(Input(
            r##"
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
            "##,
        ));
    }

    #[test]
    fn allows_test_specification_inside_a_module() {
        parse_valid(Input(
            r##"
               $"my test"{}
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod combitests {
                    #[test] fn my_test() { }
                }
            "##,
        ));
    }
    
    #[test]
    fn allows_multiple_test_specifications() {
        parse_valid(Input(
            r##"
               $"my test"{}
               $"another test"{}
               $"3rd test"{}
            "##,
        ))        
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod combitests {
                    #[test] fn my_test() { }
                    #[test] fn another_test() { }
                    #[test] fn t3rd_test() { }
                }
            "##,
        ));
    }

    #[test]
    fn preamble_is_only_included_once() {
        parse_valid(Input(
            r##"
                use mycrate::*;
                use crate::deeply::nested::{
                    my_first_function,
                    my_second_function,
                    AndATraitType
                };
               $"my test"{}
               $"another test"{}
               $"3rd test"{}
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod combitests {        
                    use mycrate::*;
                    use crate::deeply::nested::{
                        my_first_function,
                        my_second_function,
                        AndATraitType
                    };

                    #[test] fn my_test() { }
                    #[test] fn another_test() { }
                    #[test] fn t3rd_test() { }
                }   
            "##,
        ));
    }

    #[test]
    fn sparse_preamble_is_hoisted() {
        parse_valid(Input(
            r##"
                use mycrate::*;
                use crate::deeply::nested::{
                    my_first_function,
                    my_second_function,
                    AndATraitType
                };
                $"my test"{}
                use thing::*;
                $"another test"{}
                use other::*;
                $"3rd test"{}
                use some::*;
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod combitests {        
                    use mycrate::*;
                    use crate::deeply::nested::{
                        my_first_function,
                        my_second_function,
                        AndATraitType
                    };
                    use thing::*;
                    use other::*;
                    use some::*;
                    
                    #[test] fn my_test() { }
                    #[test] fn another_test() { }
                    #[test] fn t3rd_test() { }
                }   
            "##,
        ));
    }

    #[test]
    fn an_invalid_test_fails_to_compile() {
        let input = Input(
            "$"
        );
        assert!(crate::suite::process(input.stream().expect("valid token stream")).is_err());
    }

    #[test]
    fn an_invalid_test_generates_a_compile_error() {
        let input = Input(
            "$"
        );
        let err = crate::suite::process(input.stream().expect("valid token stream"));

        err.matches_failure(Expected(
            r##"
            compile_error!("expected a test name in quotes") ;
            "##))
    }
    
    fn parse_valid(input: Input) -> TestSuite {

        let mut test = TestSuite::new(NameFactory::new());
        for tok in input.stream().expect("valid token stream") {
            assert_eq!(Ok(true), test.accept_token(&tok));
        }
        test
    }
}