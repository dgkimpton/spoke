#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    use crate::{consumer::TokenStreamExt, name::*, signature::*, unit_tests::testing_helpers::*};

    #[test]
    fn can_place_an_assertion_isntead_of_a_body() {
        parse_valid(Input(
            r##"
                "test" true $eq false;
            "##,
        ))
        .generate()
        .matches(Expected(
            r##"
                #[test] 
                fn test() {
                    assert_eq!(true, false);
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
