#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    use crate::{consumer::TokenStreamExt, name::*, suite::Suite, unit_tests::testing_helpers::*};

    #[test]
    fn an_empty_input_produces_no_output() {
        parse_valid(Input(
            r##"
            "##,
        ))
        .generate()
        .matches(Expected(
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
        .generate()
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
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
        .generate()
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
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
        .generate()
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {        
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
        .generate()
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {        
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
        let (_result, error) = input.stream().process_into(Suite::new(NameFactory::new()));
        assert!(error.is_some());
    }

    fn parse_valid(input: Input) -> Suite {
        let test = Suite::new(NameFactory::new());
        let (result, _error) = input.stream().process_into(test);
        result
    }
}