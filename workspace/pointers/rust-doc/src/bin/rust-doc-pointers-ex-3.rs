
#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-pointers-bin --bin rust-doc-pointers-ex-3```
///
/// ```cargo doc  --package rust-doc-pointers-bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-pointers-bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
///
///Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.
///
///Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

/// //```rust,no_run,compile_fail
fn main(){
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{}", *ptr.offset(0) as char);
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }
}