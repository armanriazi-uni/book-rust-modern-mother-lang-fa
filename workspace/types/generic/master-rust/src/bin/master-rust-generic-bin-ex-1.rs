#![allow(dead_code, unused_variables)]
use std::fmt::Debug;
/// master-rust-generic-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-generic_bin --bin master-rust-generic-bin-ex-1```
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
/// ` 'static value passed in is: 5`
///
/// ## Example
/// //```rust,compile_fail,ignore
///   let i = 5;
///   print_it(i);
///
///   // oops, &i only has the lifetime defined by the scope of
///   // main(), so it's not 'static:
///   print_it(&i);
/// ```

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

}
