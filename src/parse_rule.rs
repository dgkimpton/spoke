use crate::{parse, parser::*};

pub(crate) trait FluidExtensions: Sized {
    fn consumed_token(self) -> ParseRule;
    fn forward_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule;
}

macro_rules! impl_rule {
    ($
        ($variant:ident),+ $(,)?
    ) => {

        pub(crate) enum ParseRule {
            $(
                $variant(parse::$variant),
            )*
        }

        impl Parser for ParseRule {
            fn accept_token(self, token: TokenTree, target: &mut SuiteGenerator) -> ParseRule {
                match self {
                    $(
                        ParseRule::$variant(rule) => rule.accept_token(token, target),
                    )*
                }
            }
            fn end_of_stream(self, target: &mut SuiteGenerator) {
                match self {
                    $(
                        ParseRule::$variant(rule) => rule.end_of_stream(target),
                    )*
                }}
            fn end_of_group(self, target: &mut SuiteGenerator)    -> ParseRule{
                match self {
                    $(
                        ParseRule::$variant(rule) => rule.end_of_group(target),
                    )*
                }}
        }

        $(
            impl FluidExtensions for parse::$variant  {
                fn consumed_token(self) -> crate::parse_rule::ParseRule {
                    crate::parse_rule::ParseRule::$variant(self)
                }
                fn forward_token(self, token:TokenTree, target:&mut SuiteGenerator) -> ParseRule {
                    self.accept_token(token, target)
                }
            }
        )+
    };
}

impl_rule!(
    Suite,
    Body,
    Assert,
    TransientSuiteAnchor,
    TransientSuiteNamed,
    TransientSuiteNamingError,
    TransientBodyAnchor,
    TransientBodyNamed,
    TransientBodyNamingError,
    TransientAssertAnchor,
    TransientAssertError,
    AssertEq,
);
