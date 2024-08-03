#![allow(dead_code, unused_variables)]


/// master-rust-smartpointer-refcell-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-smartpointer-ref-cell_bin --bin  master-rust-smartpointer-refcell-ex-1```
///
/// ```cargo doc  --package master-rust-smartpointer-ref-cell_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-smartpointer-ref-cell_bin ```
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
/// A runtime panic ! This is because of the same ownership rule of having exclusive mutable access. But, for RefCell this is checked at runtime instead. For situations like this, one has to explicitly use bare blocks to separate the borrows or use the drop method to drop the reference.
///```rust,no_run,compile_fail,ignore
/// println!("{:?} {:?}", hand1.borrow(), hand1.borrow_mut())
///```
use std::cell::RefCell;
#[derive(Debug)]
struct Bag {
    item: Box<u32>
}
fn main() {
    let bag = RefCell::new(Bag { item: Box::new(1) });
    let hand1 = &bag;
    let hand2 = &bag;
    *hand1.borrow_mut() = Bag { item: Box::new(2)};
    *hand2.borrow_mut() = Bag { item: Box::new(3)};
    let borrowed = hand1.borrow();
    println!("{:?}", borrowed);
}
