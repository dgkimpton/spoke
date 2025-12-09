# spoke::test!
Human readable test case writing for Rust

[![Crates.io](https://img.shields.io/crates/v/spoke.svg)](https://crates.io/crates/spoke)
[![docs.rs](https://img.shields.io/docsrs/spoke)](https://docs.rs/spoke/)
[![License](https://img.shields.io/crates/l/spoke.svg)](https://github.com/dgkimpton/spoke/blob/main/LICENSE)
<br>
[![Cargo Build & Test](https://github.com/dgkimpton/spoke/actions/workflows/ci.yml/badge.svg)](https://github.com/dgkimpton/spoke/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/dgkimpton/spoke/graph/badge.svg?token=GC8NVK5LSB)](https://codecov.io/github/dgkimpton/spoke)
[![dependency status](https://deps.rs/repo/github/dgkimpton/spoke/status.svg)](https://deps.rs/repo/github/dgkimpton/spoke)

> [!WARNING]  
> This library is not yet production-ready.
> 
> Feedback and suggestions welcomed

<br>

`Spoke::test!` is a proc-macro for the [Rust](https://rust-lang.org/) programming language to reduce the time and effort involved in writing tests. 

The macro transforms the simplified syntax into standard Rust `#[test]` functions but saves significant typing.

## Features

<table>
<tr>
    <td valign="top"><b>Available now</b></td>
    <td valign="top" width="50"></td>
    <td>Coming soon</td>
</tr>
<tr>
    <td>
        <ul>
            <li>Simple syntax
            <li>Test names are strings (no underscores!)
            <li>All the standard asserts
            <li>Sequential testing
            <li>Helpful compilation errors
        </ul>
    </td>
    <td></td>
    <td>These features are on the roadmap but not yet available:
        <ul>
            <li>Easy panic handling
            <li>Data based tests
            <li>Custom assert messages
            <li>Ignoring and Quarantining tests
            <li>Auto naming
        </ul>
    </td>
</tr>
</table>

for planned features see [TODO](TODO.md)

## Getting Started

Add the crate as a dependency<table>
<tr valign="top">
    <td>at the command line
    <pre>$ cargo add spoke </pre></td>
    <td width="100"></td>
    <td>
        or in <code>cargo.toml</code> add<br>
        <pre>[dependencies]
spoke = { version = "0.0.3" }</pre>
    </td>
</tr>
</table>

Then in your Rust code (e.g. `main.rs` ) you can add test cases inside a call to `spoke::test!`, e.g.

```rust
spoke::test!{
    $"result is true" true;
}
```

## The Principle
*Spoke::test!* resolves around the `$` symbol and what follows it to define nested sequential tests and assertions.

Test names are introduced as strings using `$""` which enables requirements capture without trying to introduce underscores between each word.

Nested tests concatenate the names to produce unique test names. *Spoke::test!* converts these human-readable strings into function names.

You can imagine nested tests creating a sort of tree structure, each leaf of the tree becomes a unique test function (leaves are normally assertions).

Each test name is followed by either a body `{}` or an assertion which is ended with a `;`.

Within a body any code not being preceded by a `$` is included in that test (and in any nested tests).

## Sequential Tests

Nesting requirement bodies allows for creation of sequential tests - that is, multiple tests that used the same setup but validate different assertions.


```rust
spoke::test!{
$"The user" {
    let mut user = User::new();

    $"is initially not logged in" !user.is_logged_in();

    $"can be logged in with a secret" {
        user.login("secret_token");

        $"and is then logged in" user.is_logged_in();

        $"and then logging out" {
            let result = user.logout();

            $"is ok" result;

            $"leaves the user logged out" !user.is_logged_in();
        }
    }

    $"trying to log out before login" {
        let result = user.logout();

        $"fails" !result;

        $"leaves the user still logged out" !user.is_logged_in();
    }
}
}

// becomes

#[cfg(test)]
mod spoketest {
    #[test]
    fn The_user_is_initially_not_logged_in(){
        let mut user = User::new();
        assert!(!user.is_logged_in());
    }
    #[test]
    fn The_user_can_be_logged_in_with_a_secret_and_is_then_logged_in(){
        let mut user = User::new();
        user.login("secret_token");
        assert!(user.is_logged_in());
    }
    #[test]
    fn The_user_can_be_logged_in_with_a_secret_and_then_logging_out_is_ok(){
        let mut user = User::new();
        user.login("secret_token");
        let result = user.logout();
        assert!(result);
    }
    #[test]
    fn The_user_can_be_logged_in_with_a_secret_and_then_logging_out_leaves_the_user_logged_out(){
        let mut user = User::new();
        user.login("secret_token");
        let result = user.logout();
        assert!(!user.is_logged_in());
    }
    #[test]
    fn The_user_trying_to_log_out_before_login_fails(){
        let mut user = User::new();
        let result = user.logout();
        assert!(!result);
    }
    #[test]
    fn The_user_trying_to_log_out_before_login_leaves_the_user_still_logged_out(){
        let mut user = User::new();
        let result = user.logout();
        assert!(!user.is_logged_in());
    }

}
```

### Preamble
Sometimes it is necessary to introduce use statements to pull in other crates, this can be done inside the *spoke::test!* call and is generated as an internal preamble at the start of the test module

```rust
spoke::test!{
    use std::f64::consts::*;

    $"the standard constants module" {
        $"contains a definition of Pi" PI $eq 3.14159265358979323846264338327950288_f64;
        $"contains a definition of Tau" TAU $eq 6.28318530717958647692528676655900577_f64;
    }
}

// becomes

#[cfg(test)]
mod spoketest {
    use std::f64::consts::*;

    #[test]
    fn the_standard_constants_module_contains_a_definition_of_pi(){
        assert_eq!(PI,3.14159265358979323846264338327950288_f64);
    }

    #[test]
    fn the_standard_constants_module_contains_a_definition_of_tau(){
        assert_eq!(TAU,6.28318530717958647692528676655900577_f64);
    }
}
```


## Assertions
### assert

The simplest assert is written as a named requirement followed by a boolean expression.

`$"requirement"` *&lt;expression&gt;*`;`

which maps to a standard Rust assertion like

`assert!(`*&lt;expression&gt;*`);`

and the requirement is folded into the test name.

**Simple assertion example</summary>**

```rust
$"value should be square" is_square(4);

// becomes

#[test]
fn value_should_be_square() {
    assert!(is_square(4));
}
```

### assert_eq and assert_ne

Rusts equality assertions are also supported using an infix notation `$eq` and `$ne`.


`$"requirement"` *&lt;expression_1&gt;* `$eq` *&lt;expression_2&gt;* `;`

which maps to a standard Rust assertion like

`assert_eq!(` *&lt;expression_1&gt;* `,` *&lt;expression_2&gt;* `);`

and the requirement is folded into the test name.

**Equality assertion example</summary>**

```rust
$"multiplication" {
    let a = 2;
    $"2 times 2 = 4" a*a $eq 4;
}

// becomes

#[test]
fn multiplication_2_times_2_equals_4() {
    let a = 2;
    assert_eq!(a*a,4);
}
```

## Dropping spoke::test!

So you tried *spoke::test!* and decided you don't like it? No problem.

Open your Rust file in [vscode](), navigate to the call to *spoke::test!* and then `right-click, "refactor", "inline macro"`. The call to *spoke::test!* will be replaced with the generated tests and you can then remove spoke from your `cargo.toml` and carry on as if you had never used it.

## Is this a full replacement for Rusts #[test] ?

Definitely not. *spoke::test!* is helpful in some scenarios but you should pick the testing methodology that best captures the requirements. Feel free to mix and match some tests with *spoke::test!*  and some the standard way.

## Optional Features
There are currently no optional features.

## Known Issues
Due to limitations of the proc-macro (and proc-macro2) libraries on stable some of the compile errors are highlighted against a single token when they realistically apply to multiple tokens. Improvements can be made here when the proc_macro_span feature stabilises.

If any test in a spoke requires the variable to be mutable it must be mutable for all. This can sometimes cause issues. The workaround is to split up the branches which can mean undesired duplication. Until such time as we get proper reflection I don't have a perfect solution for this (ideally the test would just drop unused mutability based on compiler feedback).

### Missing features (planned)
Currently the following features of standard Rust tests are planned but as yet unavailable.

* Ignoring tests
* Expected panics
* Changing the module name
* Custom configurations

## Feedback
If you have thoughts, suggestions, or concerns please feel free to create a [Discussion](https://github.com/dgkimpton/spoke/discussions) or directly raise an [Issue](https://github.com/dgkimpton/spoke/issues).

## License
This project uses the [MIT license](LICENSE) and that license shall automatically apply to any contributions made.

