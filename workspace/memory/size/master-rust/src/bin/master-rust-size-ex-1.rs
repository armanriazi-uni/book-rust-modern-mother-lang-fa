#![allow(dead_code, unused_variables)]

/// ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-size_bin --bin master-rust-size-ex-1```
///
/// ```cargo doc  --package master-rust-size_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-size_bin```
///
/// ## What
// `pointer_layout`
///
/// ## How
/// `This clearly shows that trait objects and references to traits are fat pointers double the size of a normal pointer.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your size/func name]
///
/// # Return
// ref_: 8
// ptr: 8
// val: 16
// pos_ref: 16
///
/// ## Example
/// //```rust,compile_fail,ignore


// pointer_layouts.rs

trait Position {}
struct Coordinates(f64, f64);
impl Position for Coordinates {}
fn main() {
    let val = Coordinates(1.0, 2.0);
    let ref_: &Coordinates = &val;
    let pos_ref: &dyn Position = &val;
    let ptr: *const Coordinates = &val;
    println!("ref_: {}", std::mem::size_of_val(&ref_));
    println!("ptr: {}", std::mem::size_of_val(&ptr));
    println!("val: {}", std::mem::size_of_val(&val));
    println!("pos_ref: {}", std::mem::size_of_val(&pos_ref));
}

 // println!("pos_ptr: {}", std::mem::size_of_val(&pos_ptr)); // Removed due to incorrect operation