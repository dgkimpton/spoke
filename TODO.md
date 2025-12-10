
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

## Async Support
The plan is to support Tokio async as the default option and use an escape hatch to customise for other runtimes. Since *Spoke::test!* is just a code generator it doesn't depending on Tokio, but if you use the default settings then your project will need to take a dependency on Tokio (or whichever other runtime you configure).

A `$async` specifier can be wrapped around a section of the test tree and will then generate all tests inside that block with a suitable async test structure.

```rust
spoke::test!{
    $"project" {
        $async {
            $"async tests inside the async block" true; 
        }
        $async(flavor = "multi_thread") {
            $"async with options" true; 
        }
        $"non-async test" true;
    }
}

// becomes
#[tokio::test]
async fn project_async_tests_inside_the_async_block() {
    assert!(true);
}

#[tokio::test(flavour="multi_thread")]
async fn project_async_with_options() {
    assert!(true);
}

#[test]
fn project_non_dash_async_test() {
    assert!(true);
}
```

By using the escape hatch the runtime can be changed

```rust
spoke::test!{
    // $config( async=block_on{/*code necessary to create the rutime which must expose a block_on method*/} );
    // or $config( async=derive=tokio::test ); //default
    // or $config( async=derive=tokio::test(flavor = "multi_thread") );
    // or $config( async=derive=my_rt::test )
    // or $config( 
    //  async=fn={
    //      Builder::new_multi_thread()
    //          .worker_threads(2)
    //        .enable_all()
    //          .build()
    //          .expect("Failed to build multi-thread runtime")
    //  }.block_on );
    // or 
    $config( async=fn=smol::block_on );

    $async {
        $"result is true" true;
    }
}

// becomes
fn result_is_true() {
    smol::block_on(async {
        assert!(true);
    });
}

```



## Assert2 Support
Eventually I'd like to add an optional feature to use [Assert2](https://github.com/de-vri-es/assert2-rs) instead of the standard `assert!` macro.