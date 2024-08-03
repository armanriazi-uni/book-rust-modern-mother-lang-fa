#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-linkedlist_bin --bin master-rust-linkedlist-main```
///
/// ```cargo doc  --package master-rust-linkedlist_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-linkedlist_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  When we instantiate any linkedlist implementing value (any heap allocated type), the Rust compiler inserts linkedlist method calls after every end of scope, after compilation. So, we don't need to manually call linkedlist on these instances. This kind of automatic reclamation based on scope is inspired by the RAII principle of C++.
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
use std::rc::Rc;
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>
}
#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    data: T
}
impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone()
            }))
        }
    }
}
fn main() {
    let list_of_nums = LinkedList::new().append(1).append(2);
    println!("nums: {:?}", list_of_nums);
    let list_of_strs = LinkedList::new().append("foo").append("bar");
    println!("strs: {:?}", list_of_strs);
}
