#![allow(dead_code, unused_variables)]

/// rust-doc-generic-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-dangle_bin --bin rust-doc-dangling-bin-ex-4```
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
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}
fn main() {
    {        
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
