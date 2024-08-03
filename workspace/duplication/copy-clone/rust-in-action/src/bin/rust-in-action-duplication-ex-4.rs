#![allow(dead_code, unused_variables)]


/// rust-in-action-duplication-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-duplication_bin --bin  rust-in-action-duplication-ex-4```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
///
fn make_nothing() -> () {
    return ();
}

/// the return type is implied as ()
/// this function will return () if nothing is specified to return
fn make_nothing2() {

}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();
    let c = 8.99f64;
    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
    println!("The value of c: {:?}", c);

     println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
    println!("The value of c: {:?}", c);
}
