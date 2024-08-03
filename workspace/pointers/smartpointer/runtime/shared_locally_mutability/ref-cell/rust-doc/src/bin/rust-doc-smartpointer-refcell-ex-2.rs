#![allow(dead_code, unused_variables)]


/// rust-doc-smartpointer-refcell-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-ref-cell_bin --bin rust-doc-smartpointer-refcell-ex-2```
///
/// ```cargo doc  --package rust-doc-smartpointer-ref-cell_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-ref-cell_bin ```
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
/// # Helper
///
/// [Reference-cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
///
/// ## Example
/// A runtime panic ! This is because of the same ownership rule of having exclusive mutable access. But, for RefCell this is checked at runtime instead. For situations like this, one has to explicitly use bare blocks to separate the borrows or use the drop method to drop the reference.
///```rust,no_run,compile_fail,ignore
/// println!("{:?} {:?}", hand1.borrow(), hand1.borrow_mut())
///```
//A reference cycle of lists a and b pointing to each other

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
fn main() {
   let data = Rc::new(RefCell::new(10));
   let _first_list = Rc::new(Cons(Rc::clone(&data), Rc::new(Nil)));
   let _second_list = Cons(Rc::new(RefCell::new(9)), Rc::clone(&_first_list));
   let _third_list = Cons(Rc::new(RefCell::new(10)), Rc::clone(&_first_list));
   *data.borrow_mut() += 20;
   println!("first list after = {:?}", _first_list);
   println!("second list after = {:?}", _second_list);
   println!("third list after = {:?}", _third_list);
}
