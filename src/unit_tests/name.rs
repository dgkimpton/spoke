#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::name::*;
    use proc_macro2::Span;

    #[test]
    fn a_name_factory_can_produce_a_simple_name() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello".into());

        assert_eq!("hello", name.function_name().expect("valid test name"))
    }

    #[test]
    fn empty_names_are_rejected() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "".into());

        assert!(name.function_name().is_err())
    }
    #[test]
    fn empty_parent_names_are_rejected() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "".into());
        let child_factory = name.make_factory();
        let child = child_factory.make_name(&Span::call_site(), "child".into());

        assert!(child.function_name().is_err())
    }
    #[test]
    fn empty_child_names_are_rejected() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "parent".into());
        let child_factory = name.make_factory();
        let child = child_factory.make_name(&Span::call_site(), "".into());

        assert!(child.function_name().is_err())
    }

    #[test]
    fn names_collapse_whitesspace_to_underscore() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello world".into());
        assert_eq!(
            "hello_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn names_collapse_multiple_whitesspace_to_a_single_underscore() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello    world".into());
        assert_eq!(
            "hello_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn names_collapse_commas_to_comma() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello,world".into());
        assert_eq!(
            "hello_comma_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn names_collapses_dot_to_dot() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello.world".into());
        assert_eq!(
            "hello_dot_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn double_equals_collapses_to_equals_equals() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello==world".into());
        assert_eq!(
            "hello_equals_equals_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn double_equals_collapses_to_equals_equals_additional_spaces_are_collapsed() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello == world".into());
        assert_eq!(
            "hello_equals_equals_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn single_equals_collapses_to_equals() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello=world".into());
        assert_eq!(
            "hello_equals_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn single_equals_collapses_to_equals_additional_spaces_are_collapsed() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello = world".into());
        assert_eq!(
            "hello_equals_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn ampersand_collapses_to_ampersand() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello&world".into());
        assert_eq!(
            "hello_ampersand_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn double_ampersand_collapses_to_ampersand_ampersand() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello&&world".into());
        assert_eq!(
            "hello_ampersand_ampersand_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn paired_brackets_collapses_to_brackets() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello [] world".into());
        assert_eq!(
            "hello_brackets_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn left_bracket_collapses_to_open_bracket() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello [ world".into());
        assert_eq!(
            "hello_open_bracket_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn right_bracket_collapses_to_close_bracket() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ] world".into());
        assert_eq!(
            "hello_close_bracket_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn paired_parenthese_collapses_to_parens() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello () world".into());
        assert_eq!(
            "hello_parens_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn left_parentesis_collapses_to_open_paren() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ( world".into());
        assert_eq!(
            "hello_open_paren_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn right_parentesis_collapses_to_close_paren() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ) world".into());
        assert_eq!(
            "hello_close_paren_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn paired_braces_collapses_to_braces() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello {} world".into());
        assert_eq!(
            "hello_braces_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn left_brace_collapses_to_open_brace() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello { world".into());
        assert_eq!(
            "hello_open_brace_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn right_brace_collapses_to_close_brace() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello } world".into());
        assert_eq!(
            "hello_close_brace_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn paired_anglebrackets_collapses_to_angle_brackets() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello <> world".into());
        assert_eq!(
            "hello_angle_brackets_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn left_angle_bracket_collapses_to_open_angle_bracket() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello < world".into());
        assert_eq!(
            "hello_open_angle_bracket_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn right_angle_bracket_collapses_to_close_angle_bracket() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello > world".into());
        assert_eq!(
            "hello_close_angle_bracket_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn paired_double_quotes_collapses_to_quotes() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello \"\" world".into());
        assert_eq!(
            "hello_quotes_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn single_double_quotes_collapses_to_quote() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello \" world".into());
        assert_eq!(
            "hello_quote_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn double_single_quotes_collapses_to_single_quotes() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello '' world".into());
        assert_eq!(
            "hello_single_quotes_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn single_single_quotes_collapses_to_single_quote() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ' world".into());
        assert_eq!(
            "hello_single_quote_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn exclamation_collapses_to_exclamation() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ! world".into());
        assert_eq!(
            "hello_exclamation_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn questionmark_collapses_to_questionmark() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ? world".into());
        assert_eq!(
            "hello_question_mark_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn at_collapses_to_at() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello @ world".into());
        assert_eq!(
            "hello_at_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn colon_collapses_to_colon() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello : world".into());
        assert_eq!(
            "hello_colon_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn semicolon_collapses_to_semicolon() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ; world".into());
        assert_eq!(
            "hello_semicolon_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn percent_collapses_to_percent() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello % world".into());
        assert_eq!(
            "hello_percent_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn hat_collapses_to_hat() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ^ world".into());
        assert_eq!(
            "hello_hat_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn star_collapses_to_star() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello * world".into());
        assert_eq!(
            "hello_star_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn slash_collapses_to_slash() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello / world".into());
        assert_eq!(
            "hello_slash_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn back_slash_collapses_to_backslash() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello \\ world".into());
        assert_eq!(
            "hello_backslash_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn plus_collapses_to_plus() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello + world".into());
        assert_eq!(
            "hello_plus_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn minus_collapses_to_minus() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello - world".into());
        assert_eq!(
            "hello_minus_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn hash_collapses_to_hash() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello # world".into());
        assert_eq!(
            "hello_hash_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn vertical_pipe_collapses_to_pipe() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello | world".into());
        assert_eq!(
            "hello_pipe_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn dollar_sign_collapses_to_dollars() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello $ world".into());
        assert_eq!(
            "hello_dollars_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn backtick_collapses_to_backtick() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ` world".into());
        assert_eq!(
            "hello_backtick_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn tilde_collapses_to_tilde() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello ~ world".into());
        assert_eq!(
            "hello_tilde_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn valid_unicode_is_kept_as_is() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello 京 world".into());
        assert_eq!(
            "hello_京_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn digits_are_kept_as_is() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello 0 world".into());
        assert_eq!(
            "hello_0_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn names_ending_in_space_are_trimmed() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello world       ".into());
        assert_eq!(
            "hello_world",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn names_ending_in_underscore_end_in_underscore() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "hello world       _".into());
        assert_eq!(
            "hello_world_",
            name.function_name().expect("valid test name")
        )
    }
    #[test]
    fn whitspace_at_the_start_is_discarded() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "    hello world".into());
        assert_eq!(
            "hello_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn when_the_first_character_of_a_name_is_not_a_valid_identifier_the_name_is_prefixed_by_t() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "    0hello world".into());
        assert_eq!(
            "t0hello_world",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn names_from_child_factories_contain_their_parents_as_prefixes() {
        let factory = NameFactory::new();
        let _ = factory.make_name(&Span::call_site(), "hello".into());
        let name = factory.make_name(&Span::call_site(), "how".into());
        let _ = factory.make_name(&Span::call_site(), "are".into());
        let _ = factory.make_name(&Span::call_site(), "you".into());

        let child_factory = name.make_factory();
        let child = child_factory.make_name(&Span::call_site(), "first".into());

        assert_eq!("how_first", child.function_name().expect("valid test name"))
    }

    #[test]
    fn names_from_many_layers_deep_child_factories_contain_their_parents_as_prefixes() {
        let factory = NameFactory::new();
        let a = factory.make_name(&Span::call_site(), "melody how".into());

        let a_child = a.make_factory();
        let b = a_child.make_name(&Span::call_site(), "are".into());

        let b_child = b.make_factory();
        let c = b_child.make_name(&Span::call_site(), "you".into());

        assert_eq!(
            "melody_how_are_you",
            c.function_name().expect("valid test name")
        )
    }

    #[test]
    fn only_the_first_name_has_its_start_prefixed_with_t_if_needed() {
        let factory = NameFactory::new();
        let a = factory.make_name(&Span::call_site(), "9melody how".into());

        let a_child = a.make_factory();
        let b = a_child.make_name(&Span::call_site(), "0".into());

        let b_child = b.make_factory();
        let c = b_child.make_name(&Span::call_site(), "1".into());

        assert_eq!(
            "t9melody_how_0_1",
            c.function_name().expect("valid test name")
        )
    }

    #[test]
    fn underscores_between_test_name_levels_are_not_collapsed_but_are_also_not_added_to() {
        let factory = NameFactory::new();
        let a = factory.make_name(&Span::call_site(), "melody_".into());

        let a_child = a.make_factory();
        let b = a_child.make_name(&Span::call_site(), "_the_".into());

        let b_child = b.make_factory();
        let c = b_child.make_name(&Span::call_site(), "_hen".into());

        assert_eq!(
            "melody__the__hen",
            c.function_name().expect("valid test name")
        )
    }

    #[test]
    fn can_ask_a_name_for_a_human_readable_fully_qualified_version() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "melody, how".into());

        assert_eq!("melody, how", name.full_name().expect("valid test name"))
    }

    #[test]
    fn can_ask_a_child_name_for_a_human_readable_fully_qualified_version() {
        let factory = NameFactory::new();
        let name = factory.make_name(&Span::call_site(), "melody, how".into());
        let child_factory = name.make_factory();
        let child = child_factory.make_name(&Span::call_site(), "first child".into());

        assert_eq!(
            "melody, how ⟶ first child",
            child.full_name().expect("valid test name")
        )
    }

    #[test]
    fn fully_qualified_names_can_go_layers_deep() {
        let factory = NameFactory::new();
        let a = factory.make_name(&Span::call_site(), "melody, how".into());

        let a_child = a.make_factory();
        let b = a_child.make_name(&Span::call_site(), "are".into());

        let b_child = b.make_factory();
        let c = b_child.make_name(&Span::call_site(), "you".into());

        assert_eq!(
            "melody, how ⟶ are ⟶ you",
            c.full_name().expect("valid test name")
        );
    }

    #[test]
    fn example_with_rust_code() {
        let factory = NameFactory::new();
        let name = factory.make_name(
            &Span::call_site(),
            "let b = a_child.make_name(&Span::call_site(), \"are\".into());".into(),
        );
        assert_eq!(
            "let_b_equals_a_child_dot_make_name_open_paren_ampersand_Span_colon_colon_call_site_parens_comma_quote_are_quote_dot_into_parens_close_paren_semicolon",
            name.function_name().expect("valid test name")
        )
    }

    #[test]
    fn names_beyond_900_characters_are_truncated_there_and_a_sequential_id_is_added_to_the_end() {
        // this is a safety feature.
        let factory = NameFactory::new();
        let name1 = factory.make_name(&Span::call_site(), "The quiet rhythm of the morning began with the low hum of the city breathing itself awake, lights flickering behind curtains, buses sighing as they eased to a stop, and people stepping carefully into the chill air that promised warmth later in the day. A baker wiped flour from his hands as the first loaves came out golden and fragrant; a jogger turned the corner, her breath tracing pale clouds behind her; and far above, in an apartment window, someone paused with coffee in hand to watch the sun edge across the horizon, gilding rooftops and antennae like patient fire. The world, unhurried but unstoppable, gathered itself again from the fragments of sleep, and each heartbeat, each sound, each unnoticed motion whispered the same simple truth: that life, for all its noise and chaos, renews itself every single morning, quietly insisting on continuing.".into());
        let name2 = factory.make_name(&Span::call_site(), "The quiet rhythm of the morning began with the low hum of the city breathing itself awake, lights flickering behind curtains, buses sighing as they eased to a stop, and people stepping carefully into the chill air that promised warmth later in the day. A baker wiped flour from his hands as the first loaves came out golden and fragrant; a jogger turned the corner, her breath tracing pale clouds behind her; and far above, in an apartment window, someone paused with coffee in hand to watch the sun edge across the horizon, gilding rooftops and antennae like patient fire. The world, unhurried but unstoppable, gathered itself again from the fragments of sleep, and each heartbeat, each sound, each unnoticed motion whispered the same simple truth: that life, for all its noise and chaos, renews itself every single morning, quietly insisting on continuing.".into());
        
        assert_eq!(
             "The_quiet_rhythm_of_the_morning_began_with_the_low_hum_of_the_city_breathing_itself_awake_comma_lights_flickering_behind_curtains_comma_buses_sighing_as_they_eased_to_a_stop_comma_and_people_stepping_carefully_into_the_chill_air_that_promised_warmth_later_in_the_day_dot_A_baker_wiped_flour_from_his_hands_as_the_first_loaves_came_out_golden_and_fragrant_semicolon_a_jogger_turned_the_corner_comma_her_breath_tracing_pale_clouds_behind_her_semicolon_and_far_above_comma_in_an_apartment_window_comma_someone_paused_with_coffee_in_hand_to_watch_the_sun_edge_across_the_horizon_comma_gilding_rooftops_and_antennae_like_patient_fire_dot_The_world_comma_unhurried_but_unstoppable_comma_gathered_itself_again_from_the_fragments_of_sleep_comma_and_each_heartbeat_comma_each_sound_comma_each_unnoticed_motion_whispered_the_same_simple_truth_colon_that_life_comma_for_all_its_noise_and_chaos_comma_renews_itse_1",
            name1.function_name().expect("valid test name")
        );

        assert_eq!(
             "The_quiet_rhythm_of_the_morning_began_with_the_low_hum_of_the_city_breathing_itself_awake_comma_lights_flickering_behind_curtains_comma_buses_sighing_as_they_eased_to_a_stop_comma_and_people_stepping_carefully_into_the_chill_air_that_promised_warmth_later_in_the_day_dot_A_baker_wiped_flour_from_his_hands_as_the_first_loaves_came_out_golden_and_fragrant_semicolon_a_jogger_turned_the_corner_comma_her_breath_tracing_pale_clouds_behind_her_semicolon_and_far_above_comma_in_an_apartment_window_comma_someone_paused_with_coffee_in_hand_to_watch_the_sun_edge_across_the_horizon_comma_gilding_rooftops_and_antennae_like_patient_fire_dot_The_world_comma_unhurried_but_unstoppable_comma_gathered_itself_again_from_the_fragments_of_sleep_comma_and_each_heartbeat_comma_each_sound_comma_each_unnoticed_motion_whispered_the_same_simple_truth_colon_that_life_comma_for_all_its_noise_and_chaos_comma_renews_itse_2",
            name2.function_name().expect("valid test name")
        )
    }
}
