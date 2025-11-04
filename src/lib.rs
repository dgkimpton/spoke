mod error;
mod span_source;
mod token_helpers;
mod name;
mod suite;
mod signature;
mod body;
mod spoke;

/// # spoke::test!
///
/// A procedural macro to simplify writing unit tests, particularly nested tests.
///
/// Add the crate as a development dependency
///
/// ```sh
/// cargo add --dev spoke
/// ```
///
/// Then in your source file add the following
///
/// ```rust
/// spoke::test!{
///     $"a vector" {
///
///         let mut v = Vec::<u8>::new();
///
///         //todo: $"starts empty" v.is_empty() $is_true;
///         //todo: $"has length zero" v.len() $eq 0;
///         //todo: $"returns nothing if popped" v.pop() $eq None;
///
///         $"when pushed to" {
///
///             v.push(8);
///
///             //todo: $"is no longer empty" v.empty() $is_false;
///             //todo: $"has length one" v.len() $eq 1;
///             //todo: $"returns the item when popped" v.pop() $eq Some(8);
///         }
///     }
/// }
/// ```
///
/// In the above snippet we have written six **seperate** unit tests covering Vector behaviour.
///
/// Spoke isn't magic, it's just syntactic sugar over the existing test framework,
/// as an example the above snippet produces:
///
/// ```
/// #[cfg(test)]
/// #[allow(unused_mut)]
/// #[allow(unused_variables)]
/// mod spoke_tests {
///     #[test]
///     fn a_vector_starts_empty() {
///         let mut v = Vec::<u8>::new();
///         assert!(v.is_empty());
///     }
///     #[test]
///     fn a_vector_has_length_zero() {
///         let mut v = Vec::<u8>::new();
///         assert_eq!(v.len(), 0);
///     }
///     #[test]
///     fn a_vector_returns_nothing_if_popped() {
///         let mut v = Vec::<u8>::new();
///         assert_eq!(v.pop(), None);
///     }
///     #[test]
///     fn a_vector_when_pushed_to_is_no_longer_empty() {
///         let mut v = Vec::<u8>::new();
///         v.push(8);
///         assert!(!(v.is_empty()));
///     }
///     #[test]
///     fn a_vector_when_pushed_to_has_length_one() {
///         let mut v = Vec::<u8>::new();
///         v.push(8);
///         assert_eq!(v.len(), 1);
///     }
///     #[test]
///     fn a_vector_when_pushed_to_returns_the_item_when_popped() {
///         let mut v = Vec::<u8>::new();
///         v.push(8);
///         assert_eq!(v.pop(), Some(8));
///     }
/// }
/// ```
///
/// Hopefully you can see how it is easier to get the requirements down quickly using spoke.
/// And, if you later decide not to stick with it, vscode has a helpful feature where you can
/// `right-click, "refactor", "inline macro"` which will leave you with normal rust tests and no trace of spoke.
///
/// Not every test needs to, or benefits from, being written with spoke, but for simple sequential tests it can help
/// you get up and running quickly.
///
#[proc_macro]
pub fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::spoke::generate_tests(proc_macro2::TokenStream::from(input)).into()
}

#[cfg(test)]
mod unit_tests;
