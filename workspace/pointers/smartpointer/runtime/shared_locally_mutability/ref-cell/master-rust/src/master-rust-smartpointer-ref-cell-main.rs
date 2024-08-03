#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-smartpointer-ref-cell_bin --bin master-rust-smartpointer-ref-cell-main```
///
/// ```cargo doc  --package master-rust-smartpointer-ref-cell_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-smartpointer-ref-cell_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This works as you would expect, and the only added cost is that you have to write a bit more. The additional runtime cost is zero, though, and the references to the mutable things remain immutable.`
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
/// ```rust,compile_fail,no_run,ignore
/// use std::cell::Cell;
/// #[derive(Debug)]
/// struct Bag {
///     item: Box<u32>
/// }
/// fn main() {
///     let mut bag = Cell::new(Bag { item: Box::new(1) });
///     let hand1 = &mut bag;
///     let hand2 = &mut bag; //Expose error second mutable borrow occurs here
///     *hand1 = Cell::new(Bag {item: Box::new(2)});
///     *hand2 = Cell::new(Bag {item: Box::new(2)});
/// }
/// ```
use std::cell::Cell;
#[derive(Debug)]
struct Bag {
    item: Box<u32>
}
fn main() {
    let bag = Cell::new(Bag { item: Box::new(1) });
    let hand1 = &bag;
    let hand2 = &bag;
    hand1.set(Bag { item: Box::new(2)});
    hand2.set(Bag { item: Box::new(3)});
    println!("{:?}",bag.into_inner());
}
