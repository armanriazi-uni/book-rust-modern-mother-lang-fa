#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-linkedlist_bin --bin rust-doc-linkedlist-main```
///
/// ```cargo doc  --package rust-doc-linkedlist_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-linkedlist_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  When we instantiate any linkedlist implementing value (any heap allocated type), the Rust compiler inserts linkedlist method calls after every end of scope, after compilation. So, we don't need to manually call linkedlist on these instances. This kind of automatic reclamation based on scope is inspired by the RAII principle of C++.
/// That changes a bit if we want to keep the structure this simple but still have a double-linked list, since then we actually have to change the existing structure.
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
use crate::List::*;
enum List {
    Cons(u32, Box<List>),
    Nil,
}
impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons( _,ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
