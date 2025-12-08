
# Planned Features


## Easy panic handling
Sometimes it is needed to verify that a test panics. The planned syntax for this is expected to be:

```rust
$"it is not possible to do it" do_it() $panics;
$"it is not possible to do it" do_it() $panics "string to match";

// expands using rusts $[panic(reason)] macro
```


## Data based tests
Frequently we want to write the same test with different input values, this is tedious copy paste. *Spoke::test!* plans to have the following data drivent syntax


```rust
$(a, b, c)
 [3, 4, 12]
 [3, 3, 9]
 [1, 2, 2]
 [0, 10, 0]
"given {a} and {b} as input, the output is expected to be {c}" a*b $eq c;

// expands into multiple tests, each one testing one row of input

#[test]
fn given_3_and_4_as_input_comma_the_output_is_expected_to_be_12(){
    let a = 3;
    let b = 4;
    let c = 12;    
    assert!(a*b, c);
}
#[test]
fn given_3_and_3_as_input_comma_the_output_is_expected_to_be_9(){
    let a = 3;
    let b = 3;
    let c = 9;    
    assert!(a*b, c);
}
#[test]
fn given_1_and_2_as_input_comma_the_output_is_expected_to_be_2(){
    let a = 1;
    let b = 2;
    let c = 2;    
    assert!(a*b, c);
}
#[test]
fn given_0_and_10_as_input_comma_the_output_is_expected_to_be_0(){
    let a = 0;
    let b = 10;
    let c = 0;    
    assert!(a*b, c);
}

```

**Also supports differing types and type specification**

```rust
$(a:u8, b:u8, c:usize)
 [1,2,3]
 "using argument types to constrain the variable definition"{
    //...
 }

$(a)
 [Blob()]
 [Thing()]
 "using inference to get differing types into the test"{
    //...
 }
```

## Custom assert messages
Rust testing macros allow for a custom message on error, *Spoke::test!* plans to allow for this too using the following syntax

```rust
$"no items in cart" basket.is_empty() $onfail "the cart still had {} items in it", basket.len();
```

## Ignoring and Pending tests
Rust can ignore tests and *Spoke::test!* plans to be able to too

```rust
$ignore "no items in cart" basket.is_empty();
$ignore."reason" "no items in cart" basket.is_empty();
```

Some testing frameworks also support quarantining failing tests which will then appear as if they pass until such time as they genuinely pass at which point they will fail.  This lets users ingore them but with the bonus that when the pass they are prompted to clean up the failure expectation.

This is really a question of marking a test as pending complete development.

A more Rusty way to handle this is to put the test behind a feature-flag and then exclude that from the command line when running the test suite.

The planned syntax for allowing this approach is

```rust
$pending "no items in cart" basket.is_empty();
$pending.feature_name "no items in cart" basket.is_empty();
```

This requires some manual editing of the cargo.toml so it's still in the ideation phase.


## Auto naming
Using an assertion name of `$$` will convert the content of the assert into a function name.

This avoids the need to duplicate obvious code in English. 

E.g.
```rust
$$ basket.is_empty();

// instead of 
$"the basket is empty" basket.is_empty();
```

## Escape Hatches
Currently the *Spoke::test!* generated module is always called `spoketest`, very occasionally it might be needed to change this. There should be a syntax that allows for that.

Also, *Spoke::test!* gnerally pulls in `super::*` automatically which isn't to everyone's taste, there should be an option to avoid this.

Proposed syntax is to use a `$config` action in the pre-amble.

```rust
spoke::test!{
    $config( module = mymod, super = off );

    $"result is true" true;
}
```

## Assert2 Support
Eventually I'd like to add an optional feature to use [Assert2](https://github.com/de-vri-es/assert2-rs) instead of the standard `assert!` macro.