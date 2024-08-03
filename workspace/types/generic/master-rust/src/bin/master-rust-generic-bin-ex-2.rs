#![allow(dead_code, unused_variables)]

/// master-rust-generic-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-generic_bin --bin master-rust-generic-bin-ex-2```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `static_string: I'm in read-only memory`
/// `coerced_static: 18`
/// `NUM: 18 stays accessible!`
///
/// ## Example
/// //```rust
/// fn coerce_static<'a>(u: &'a i32) -> &'a i32 {
///     &u //return 9
/// }

static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
