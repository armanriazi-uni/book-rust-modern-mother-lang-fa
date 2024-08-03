#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-ref-cell_bin --bin rust-doc-smartpointer-refcell-ex-1```
///
/// ```cargo doc  --package rust-doc-smartpointer-ref-cell_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-ref-cell_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  `TODO`
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
/// //rust,compile_fail,no_run,ignore
///  `TODO`
///
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
//RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.
//So this exmple show have many immutable borrows
//Mutating the value inside an immutable value is the interior mutability pattern.
//So with *value.borrow_mut() += num we had using the interior mutability pattern.

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let d = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));


    *value.borrow_mut() += 10;
    println!("counter a after = {:?}", a);
    println!("b after = {:?}", b);

    *value.borrow_mut() += 10;
    println!("c after = {:?}", c);
    {
        *value.borrow_mut() += 10;
         println!("(inner scope)d after = {:?}", d);
         println!("(inner scope)a strong_count = {}", Rc::strong_count(&a));
    }
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!("d after = {:?}", d);
    let e = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));
    println!("strong_count after c goes out of scope = {}", Rc::strong_count(&a));

    *value.borrow_mut() += 10;
    println!("e after = {:?}", e);
}
