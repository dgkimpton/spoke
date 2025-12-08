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

    fn parse_valid(input: Input) -> proc_macro2::TokenStream {
        let mut output = SuiteGenerator::new();

        let tok = Input(format!("{{ {} }}", input.0).as_str())
            .stream()
            .into_iter()
            .next()
            .expect("there should be valid input");

        match tok {
            proc_macro2::TokenTree::Group(group) => parse::Body::from_suite(
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