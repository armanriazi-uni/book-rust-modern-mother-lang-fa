#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-smartpointer-box_bin --bin master-rust-smartpointer-box-main```
///
/// ```cargo doc  --package master-rust-smartpointer-box_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-smartpointer-box_bin```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// ````rust,compile_fail,no_run,ignore
/// struct Node {
///     data: u32,
///     next: Option<Node> //  next: Option<Box<Node>>
///                    // ++++    +
/// }
/// fn main() {
///     let a = Node { data: 33, next: None };
/// }
/// ```
/// `Output` error recursive type has infinite size
///


struct Node {
    data: u32,
    next: Option<Box<Node>>
}

fn main(){

     let a = Node { data: 33, next: None };
}
