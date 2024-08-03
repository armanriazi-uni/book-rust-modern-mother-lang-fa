#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-rc_bin --bin rust-doc-rc-ex-1```
///
/// ```cargo doc  --package rust-doc-rc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-rc_bin ```
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

/*
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}*/
/*We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case. The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do. The call to Rc::clone only increments the reference count, which doesn’t take much time.
 When c goes out of scope, the count goes down by 1-above
he implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.


*/
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let da=Rc::downgrade(&a);
    println!("count after creating a = {}", Rc::weak_count(&a));
    let da=Rc::downgrade(&a);
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::weak_count(&a));
    {   let da=Rc::downgrade(&a);
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::weak_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::weak_count(&a));
     let da=Rc::downgrade(&a);
}


