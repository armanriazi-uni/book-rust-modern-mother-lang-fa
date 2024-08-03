#![allow(dead_code, unused_variables)]

/// ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-tuple_bin --bin rust-doc-tuple-ex-3```
///
/// ```cargo doc  --package rust-doc-tuple_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-tuple_bin```
///
/// ## What
// `rust-doc_variables_shadowing`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your tuple/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
fn main() {
    // rust infers the type of x
    let  x = 13;
    //x=13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 =  std::f64::consts::PI;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);
}
