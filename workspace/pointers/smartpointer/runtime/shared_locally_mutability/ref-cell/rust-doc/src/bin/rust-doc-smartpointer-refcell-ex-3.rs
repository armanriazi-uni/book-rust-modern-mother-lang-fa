#![allow(dead_code, unused_variables)]


/// rust-doc-smartpointer-refcell-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-ref-cell_bin --bin rust-doc-smartpointer-refcell-ex-3```
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
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("--a = {:?}", a);
    println!("--a.taild() = {:?}", a.tail());

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("--a= {:?}", a);
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("--b= {:?}", b);
    println!("--b.tail()= {:?}", b.tail()); 

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("--a = {:?}", a);
    println!("--a.taild() = {:?}", a.tail());

    println!("b initial rc count = {}", Rc::strong_count(&b));

    println!("--b= {:?}", b);
    println!("b next item = {:?}", b.tail());
    println!("\n");
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {:#}", Rc::strong_count(&b));
    println!("a rc count after changing a = {:#}", Rc::strong_count(&a));
}
