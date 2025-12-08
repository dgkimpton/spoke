#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::name::*;
    use proc_macro2::Span;

    #[test]
    fn a_name_name_can_produce_a_simple_name() {
        let names = [Name::new(&Span::call_site(), "hello")];
        let name = CompoundName::new().followed_by(&names[0]);

        assert_eq!("hello", name.function_name().0)
    }

    #[test]
    fn totall_empty_names_are_collapsed_empty() {
        let names = [Name::new(&Span::call_site(), "")];
        let name = CompoundName::new().followed_by(&names[0]);

        assert_eq!("", name.function_name().0)
    }
    #[test]
    fn empty_parent_names_are_collapsed() {
        let names = [
            Name::new(&Span::call_site(), ""),
            Name::new(&Span::call_site(), "child"),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1]);

        assert_eq!("child", name.function_name().0)
    }
    #[test]
    fn empty_child_names_are_collapsed_to_underscore() {
        let names = [
            Name::new(&Span::call_site(), "parent"),
            Name::new(&Span::call_site(), ""),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1]);

        assert_eq!("parent_", name.function_name().0)
    }

    #[test]
    fn names_collapse_whitesspace_to_underscore() {
        let names = [Name::new(&Span::call_site(), "hello world")];
        let name = CompoundName::new().followed_by(&names[0]);

        assert_eq!("hello_world", name.function_name().0)
    }

    #[test]
    fn names_collapse_multiple_whitesspace_to_a_single_underscore() {
        let names = [Name::new(&Span::call_site(), "hello    world")];
        let name = CompoundName::new().followed_by(&names[0]);

        assert_eq!("hello_world", name.function_name().0)
    }

    #[test]
    fn names_collapse_commas_to_comma() {
        let names = [Name::new(&Span::call_site(), "hello,world")];
        let name = CompoundName::new().followed_by(&names[0]);

        assert_eq!("hello_comma_world", name.function_name().0)
    }

    #[test]
    fn names_collapses_dot_to_dot() {
        let names = [Name::new(&Span::call_site(), "hello.world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_dot_world", name.function_name().0)
    }
    #[test]
    fn double_equals_collapses_to_equals_equals() {
        let names = [Name::new(&Span::call_site(), "hello==world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_equals_equals_world", name.function_name().0)
    }
    #[test]
    fn double_equals_collapses_to_equals_equals_additional_spaces_are_collapsed() {
        let names = [Name::new(&Span::call_site(), "hello == world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_equals_equals_world", name.function_name().0)
    }
    #[test]
    fn single_equals_collapses_to_equals() {
        let names = [Name::new(&Span::call_site(), "hello=world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_equals_world", name.function_name().0)
    }
    #[test]
    fn single_equals_collapses_to_equals_additional_spaces_are_collapsed() {
        let names = [Name::new(&Span::call_site(), "hello = world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_equals_world", name.function_name().0)
    }
    #[test]
    fn ampersand_collapses_to_ampersand() {
        let names = [Name::new(&Span::call_site(), "hello&world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_ampersand_world", name.function_name().0)
    }
    #[test]
    fn double_ampersand_collapses_to_ampersand_ampersand() {
        let names = [Name::new(&Span::call_site(), "hello&&world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_ampersand_ampersand_world", name.function_name().0)
    }

    #[test]
    fn paired_brackets_collapses_to_brackets() {
        let names = [Name::new(&Span::call_site(), "hello [] world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_brackets_world", name.function_name().0)
    }
    #[test]
    fn left_bracket_collapses_to_open_bracket() {
        let names = [Name::new(&Span::call_site(), "hello [ world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_open_bracket_world", name.function_name().0)
    }
    #[test]
    fn right_bracket_collapses_to_close_bracket() {
        let names = [Name::new(&Span::call_site(), "hello ] world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_close_bracket_world", name.function_name().0)
    }

    #[test]
    fn paired_parenthese_collapses_to_parens() {
        let names = [Name::new(&Span::call_site(), "hello () world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_parens_world", name.function_name().0)
    }
    #[test]
    fn left_parentesis_collapses_to_open_paren() {
        let names = [Name::new(&Span::call_site(), "hello ( world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_open_paren_world", name.function_name().0)
    }
    #[test]
    fn right_parentesis_collapses_to_close_paren() {
        let names = [Name::new(&Span::call_site(), "hello ) world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_close_paren_world", name.function_name().0)
    }

    #[test]
    fn paired_braces_collapses_to_braces() {
        let names = [Name::new(&Span::call_site(), "hello {} world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_braces_world", name.function_name().0)
    }
    #[test]
    fn left_brace_collapses_to_open_brace() {
        let names = [Name::new(&Span::call_site(), "hello { world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_open_brace_world", name.function_name().0)
    }
    #[test]
    fn right_brace_collapses_to_close_brace() {
        let names = [Name::new(&Span::call_site(), "hello } world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_close_brace_world", name.function_name().0)
    }

    #[test]
    fn paired_anglebrackets_collapses_to_angle_brackets() {
        let names = [Name::new(&Span::call_site(), "hello <> world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_angle_brackets_world", name.function_name().0)
    }
    #[test]
    fn left_angle_bracket_collapses_to_open_angle_bracket() {
        let names = [Name::new(&Span::call_site(), "hello < world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_open_angle_bracket_world", name.function_name().0)
    }
    #[test]
    fn right_angle_bracket_collapses_to_close_angle_bracket() {
        let names = [Name::new(&Span::call_site(), "hello > world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_close_angle_bracket_world", name.function_name().0)
    }

    #[test]
    fn paired_double_quotes_collapses_to_quotes() {
        let names = [Name::new(&Span::call_site(), "hello \"\" world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_quotes_world", name.function_name().0)
    }
    #[test]
    fn single_double_quotes_collapses_to_quote() {
        let names = [Name::new(&Span::call_site(), "hello \" world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_quote_world", name.function_name().0)
    }
    #[test]
    fn double_single_quotes_collapses_to_single_quotes() {
        let names = [Name::new(&Span::call_site(), "hello '' world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_single_quotes_world", name.function_name().0)
    }
    #[test]
    fn single_single_quotes_collapses_to_single_quote() {
        let names = [Name::new(&Span::call_site(), "hello ' world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_single_quote_world", name.function_name().0)
    }

    #[test]
    fn exclamation_collapses_to_exclamation() {
        let names = [Name::new(&Span::call_site(), "hello ! world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_exclamation_world", name.function_name().0)
    }

    #[test]
    fn questionmark_collapses_to_questionmark() {
        let names = [Name::new(&Span::call_site(), "hello ? world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_question_mark_world", name.function_name().0)
    }

    #[test]
    fn at_collapses_to_at() {
        let names = [Name::new(&Span::call_site(), "hello @ world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_at_world", name.function_name().0)
    }

    #[test]
    fn colon_collapses_to_colon() {
        let names = [Name::new(&Span::call_site(), "hello : world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_colon_world", name.function_name().0)
    }

    #[test]
    fn semicolon_collapses_to_semicolon() {
        let names = [Name::new(&Span::call_site(), "hello ; world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_semicolon_world", name.function_name().0)
    }

    #[test]
    fn percent_collapses_to_percent() {
        let names = [Name::new(&Span::call_site(), "hello % world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_percent_world", name.function_name().0)
    }
    #[test]
    fn hat_collapses_to_hat() {
        let names = [Name::new(&Span::call_site(), "hello ^ world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_hat_world", name.function_name().0)
    }

    #[test]
    fn star_collapses_to_star() {
        let names = [Name::new(&Span::call_site(), "hello * world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_star_world", name.function_name().0)
    }

    #[test]
    fn slash_collapses_to_slash() {
        let names = [Name::new(&Span::call_site(), "hello / world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_slash_world", name.function_name().0)
    }
    #[test]
    fn back_slash_collapses_to_backslash() {
        let names = [Name::new(&Span::call_site(), "hello \\ world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_backslash_world", name.function_name().0)
    }
    #[test]
    fn plus_collapses_to_plus() {
        let names = [Name::new(&Span::call_site(), "hello + world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_plus_world", name.function_name().0)
    }
    #[test]
    fn minus_collapses_to_minus() {
        let names = [Name::new(&Span::call_site(), "hello - world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_minus_world", name.function_name().0)
    }
    #[test]
    fn hash_collapses_to_hash() {
        let names = [Name::new(&Span::call_site(), "hello # world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_hash_world", name.function_name().0)
    }
    #[test]
    fn vertical_pipe_collapses_to_pipe() {
        let names = [Name::new(&Span::call_site(), "hello | world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_pipe_world", name.function_name().0)
    }
    #[test]
    fn dollar_sign_collapses_to_dollars() {
        let names = [Name::new(&Span::call_site(), "hello $ world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_dollars_world", name.function_name().0)
    }

    #[test]
    fn backtick_collapses_to_backtick() {
        let names = [Name::new(&Span::call_site(), "hello ` world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_backtick_world", name.function_name().0)
    }

    #[test]
    fn tilde_collapses_to_tilde() {
        let names = [Name::new(&Span::call_site(), "hello ~ world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_tilde_world", name.function_name().0)
    }

    #[test]
    fn valid_unicode_is_kept_as_is() {
        let names = [Name::new(&Span::call_site(), "hello 京 world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_京_world", name.function_name().0)
    }

    #[test]
    fn digits_are_kept_as_is() {
        let names = [Name::new(&Span::call_site(), "hello 0 world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_0_world", name.function_name().0)
    }

    #[test]
    fn names_ending_in_space_are_trimmed() {
        let names = [Name::new(&Span::call_site(), "hello world       ")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_world", name.function_name().0)
    }
    #[test]
    fn names_ending_in_underscore_end_in_underscore() {
        let names = [Name::new(&Span::call_site(), "hello world       _")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_world_", name.function_name().0)
    }
    #[test]
    fn whitspace_at_the_start_is_discarded() {
        let names = [Name::new(&Span::call_site(), "    hello world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("hello_world", name.function_name().0)
    }

    #[test]
    fn when_the_first_character_of_a_name_is_not_a_valid_identifier_the_name_is_prefixed_by_t() {
        let names = [Name::new(&Span::call_site(), "    0hello world")];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!("t0hello_world", name.function_name().0)
    }

    #[test]
    fn names_from_child_factories_contain_their_parents_as_prefixes() {
        let names = [
            Name::new(&Span::call_site(), "how"),
            Name::new(&Span::call_site(), "first"),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1]);

        assert_eq!("how_first", name.function_name().0)
    }

    #[test]
    fn names_from_many_layers_deep_child_factories_contain_their_parents_as_prefixes() {
        let names = [
            Name::new(&Span::call_site(), "melody how"),
            Name::new(&Span::call_site(), "are"),
            Name::new(&Span::call_site(), "you"),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1])
            .followed_by(&names[2]);

        assert_eq!("melody_how_are_you", name.function_name().0)
    }

    #[test]
    fn only_the_first_name_has_its_start_prefixed_with_t_if_needed() {
        let names = [
            Name::new(&Span::call_site(), "9melody how"),
            Name::new(&Span::call_site(), "0"),
            Name::new(&Span::call_site(), "1"),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1])
            .followed_by(&names[2]);

        assert_eq!("t9melody_how_0_1", name.function_name().0)
    }

    #[test]
    fn underscores_between_test_name_levels_are_collapsed() {
        let names = [
            Name::new(&Span::call_site(), "melody_"),
            Name::new(&Span::call_site(), "_the_"),
            Name::new(&Span::call_site(), "_hen"),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1])
            .followed_by(&names[2]);

        assert_eq!("melody_the_hen", name.function_name().0)
    }

    #[test]
    fn example_with_rust_code() {
        let names = [Name::new(
            &Span::call_site(),
            "let b = a_child.followed_by(Name::new(&Span::call_site(), \"are\"));",
        )];
        let name = CompoundName::new().followed_by(&names[0]);
        assert_eq!(
            "let_b_equals_a_child_dot_followed_by_open_paren_name_colon_colon_new_open_paren_ampersand_span_colon_colon_call_site_parens_comma_quote_are_quote_close_paren_close_paren_semicolon",
            name.function_name().0
        )
    }

    #[test]
    fn can_create_missing_names() {
        let names = [
            Name::missing(&Span::call_site(), 1),
            Name::missing(&Span::call_site(), 2),
            Name::missing(&Span::call_site(), 3),
        ];
        let name = CompoundName::new()
            .followed_by(&names[0])
            .followed_by(&names[1])
            .followed_by(&names[2]);

        assert_eq!("missing_name_missing_name_2_missing_name_3", name.function_name().0)
    }
}
