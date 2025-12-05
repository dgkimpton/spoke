#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::{spoke, unit_tests::testing_helpers::*};

    #[test]
    fn an_empty_input_produces_no_output() {
        parsing(Input(
            r##"
            "##,
        ))
        .matches(Expected(
            r##"
            "##,
        ));
    }

    #[test]
    fn an_empty_test_body_produces_a_single_test() {
        parsing(Input(
            r##"
            $"first test" {}
            "##,
        ))
        .matches(Expected(
            r##"
            #[cfg(test)]
            #[allow(unused_mut)]
            #[allow(unused_variables)] 
            mod spoketest {
                #[test] fn first_test() { }
            }
            "##,
        ));
    }

    #[test]
    fn allows_multiple_test_specifications() {
        parsing(Input(
            r##"
               $"my test"{}
               $"another test"{}
               $"3rd test"{}
            "##,
        ))
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
    fn preamble_is_included() {
        parsing(Input(
            r##"
            use mycrate::*;
            use crate::deeply::nested::{
                my_first_function,
                my_second_function,
                AndATraitType
            };
            $"first test" {}
            "##,
        ))
        .matches(Expected(
            r##"
            #[cfg(test)]
            #[allow(unused_mut)]
            #[allow(unused_variables)] 
                mod spoketest {
                use mycrate::*;
                use crate::deeply::nested::{
                    my_first_function,
                    my_second_function,
                    AndATraitType
                };

                #[test] fn first_test() { }
            }
            "##,
        ));
    }

    #[test]
    fn preamble_is_only_included_once() {
        parsing(Input(
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
        parsing(Input(
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
    fn after_a_broken_name_test_body_is_still_produced() {
        parsing(Input(
            r##"
            $"first test"uhoh {}
            "##,
        ))
        .matches(Expected(
            r##"
            #[cfg(test)]
            #[allow(unused_mut)]
            #[allow(unused_variables)] 
            mod spoketest {
                compile_error!("expected a valid test case name in quotes following the dollars, but found `\"first test\"uhoh`\nunmatched suffix detected, `uhoh`, did you miss a space?");
                #[test] fn missing_name() { }
            }
            "##,
        ));
    }

    #[test]
    fn after_a_broken_name_test_body_is_still_produced_2() {
        parsing(Input(
            r##"
            $first test {}
            "##,
        ))
        .matches(Expected(
            r##"
            #[cfg(test)]
            #[allow(unused_mut)]
            #[allow(unused_variables)] 
            mod spoketest {
                compile_error!("expected a valid test case name in quotes following the dollars, but found `first`");
                #[test] fn missing_name() { }
            }
            "##,
        ));
    }

    fn parsing(input: Input) -> proc_macro2::TokenStream {
        spoke::generate_tests(input.stream())
    }
}

#[cfg(test)]
mod error_tests {
    #[allow(unused_imports)]
    use super::*;

    use crate::{spoke, unit_tests::testing_helpers::*};

    #[test]
    fn missing_name_empty_test() {
        parsing(Input(
            r##"
               $;
            "##,
        ))
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("expected a valid test case name in quotes following the dollars, but found `;` before any body was provided");
                }
            "##,
        ));
    }
    #[test]
    fn missing_name_due_to_eof() {
        parsing(Input(
            r##"
               $
            "##,
        ))
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("reached end of input before reaching the end of the test definition");
                }
            "##,
        ));
    }

    #[test]
    fn invalid_name_token_empty_test() {
        parsing(Input(
            r##"
               $hello world;
            "##,
        ))
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("expected a valid test case name in quotes following the dollars, but found `hello`");
                }
            "##,
        ));
    }

    #[test]
    fn invalid_name_token_stream_end() {
        parsing(Input(
            r##"
               $hello
            "##,
        ))
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("expected a valid test case name in quotes following the dollars, but found `hello`");
                    compile_error!("reached end of input before reaching the end of the test definition");
                }
            "##,
        ));
    }

    #[test]
    fn missing_top_level_test_body() {
        parsing(Input(
            r##"
               $"hello";
            "##,
        ))
        .matches(Expected(
            r##"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("expected a test body in braces {} after the name but found `;`");
                }
            "##,
        ));
    }

    #[test]
    fn invalid_token_directly_following_name() {
        parsing(Input(
            r##"
               $"hello"world;
            "##,
        ))
        .matches(Expected(
            r###"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("expected a valid test case name in quotes following the dollars, but found `\"hello\"world`\nunmatched suffix detected, `world`, did you miss a space?");
                }
            "###,
        ));
    }

    #[test]
    fn eof_token_directly_following_name() {
        parsing(Input(
            r##"
               $"hello"
            "##,
        ))
        .matches(Expected(
            r###"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("reached end of input before finding the test body for named test");
                }
            "###,
        ));
    }

    #[test]
    fn valid_name_then_error_token_then_eof() {
        parsing(Input(
            r##"
               $"hello" #
            "##,
        ))
        .matches(Expected(
            r###"
                #[cfg(test)]
                #[allow (unused_mut)]
                #[allow (unused_variables)] 
                mod spoketest {
                    compile_error!("expected a test body in braces {} after the name but found `#`") ;
                    compile_error!("reached end of input before reaching the end of the test definition");
                }
            "###,
        ));
    }

    fn parsing(input: Input) -> proc_macro2::TokenStream {
        spoke::generate_tests(input.stream())
    }
}
