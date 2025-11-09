#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    use crate::{name::*, signature::*, unit_tests::testing_helpers::*};

    #[test]
    fn can_place_an_assertion_isntead_of_a_body() {
        parse_valid(Input(
            r##"
                "test" true $eq false;
            "##,
        ))
        .generate_tokens()
        .matches_ok(Expected(
            r##"
                #[test] 
                fn test() {
                    assert_eq!(true, false);
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
