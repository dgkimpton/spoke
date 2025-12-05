use std::fmt::Display;

use proc_macro2::TokenTree;

use crate::{name::Name, string_lit::TokenExtensions};

pub(crate) enum Dollars {
    AssertEq,
}

impl Display for Dollars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dollars::AssertEq => write!(f, "eq"),
        }
    }
}

pub(crate) enum MatchResult {
    Name(Name),
    InvalidName(String),
    ValidDollars(Dollars),
    InvalidDollars(Dollars),
    OtherInvalid(String),
    SemiColon,
}

pub(crate) fn dollars_match(tok: &TokenTree) -> MatchResult {
    let sp = tok.span();

    match tok {
        TokenTree::Literal(literal) => match literal.as_string_literal() {
            Ok(literal) => MatchResult::Name(Name::new(&sp, literal)),

            Err(error) => MatchResult::InvalidName(format!("found `{}`\n{}", literal, error)),
        },

        TokenTree::Ident(ident) => match dollars_ident_match(ident.to_string()) {
            Ok(dollars) => MatchResult::ValidDollars(dollars),
            Err(rejected) => match dollars_ident_match(rejected.to_ascii_lowercase()) {
                Ok(dollars) => MatchResult::InvalidDollars(dollars),
                Err(rejected) => MatchResult::OtherInvalid(rejected),
            },
        },
        TokenTree::Punct(punct) if punct.as_char() == ';' => MatchResult::SemiColon,

        other => MatchResult::OtherInvalid(other.to_string()),
    }
}

pub(crate) fn dollars_ident_match(ident: String) -> Result<Dollars, String> {
    match ident.as_str() {
        "eq" => Result::Ok(Dollars::AssertEq),
        other => Result::Err(other.to_string()),
    }
}
