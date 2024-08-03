#![allow(dead_code, unused_variables)]

/// rust-doc-elision-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-elision_bin --bin  rust-doc-elision-bin-ex-1```
///
/// ## What
/// `TODO`
///
/// ## How
/// ` elided_input and annotated_input essentially have identical signatures
// because the lifetime of elided_input is inferred by the compiler
/// Similarly, elided_pass and annotated_pass have identical signatures
/// because the lifetime is added implicitly to elided_pass`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `elided_input: 3
///annotated_input: 3
///elided_pass: 3
///annotated_pass: 3`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore

fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}
fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}
fn elided_pass(x: &i32) -> &i32 { x }
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }
fn main() {
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
