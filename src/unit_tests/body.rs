#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    use crate::{name::*, body::TestBody, unit_tests::testing_helpers::*};

    #[test]
    fn keeps_basic_code() {
        parse_valid(Input(
            r##"
                let x = 5;
                let y = 5;
                assert_eq!(x,y);
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
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
        .generate_tokens()
        .matches_ok(Expected(
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
    fn nested_tests_keep_their_respective_bodies(){
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
        .generate_tokens()
        .matches_ok(Expected(
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
        .generate_tokens()
        .matches_ok(Expected(
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
        .generate_tokens()
        .matches_ok(Expected(
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

    fn parse_valid(input: Input) -> TestBody {
        let sp = proc_macro2::Span::call_site();
        let nf = NameFactory::new();
        let name = nf.make_name(&sp, "a test".to_string());
        let child_nf = name.make_factory();

        let mut test = TestBody::new(name, child_nf,vec![]);
        for tok in input.stream().expect("valid token stream") {
            assert_eq!(Ok(true), test.accept_token(&tok));
        }
        test
    }
}