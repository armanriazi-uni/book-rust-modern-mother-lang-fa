
#![allow(dead_code, unused_variables)]
// use std::rc::Rc;
// use std::rc::Weak;
// use std::cell::RefCell;


/// mater-rust-linkedlist-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-linkedlist_bin --bin  mater-rust-linkedlist-ex-1```
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
// /// ```rust,no_run,compile_fail,ignore
// use std::rc::Rc;
// use std::rc::Weak;
// use std::cell::RefCell;
// use std::rc::Weak;
//  impl<T> LinkedList<T> {
//      fn new() -> Self {
//          LinkedList { head: None }
//      }
//      fn append(&mut self, data: T) -> Self {
//          let new_node = Rc::new(LinkedListNode {
//              data: data,
//              next: self.head.clone(),
//              prev: None
//          });
//          match self.head.clone() {
//              Some(node) => {
//                  node.prev = Some(Rc::downgrade(&new_node)); // Exposed error cannot assign to node.prev
//              },
//              None => {
//              }
//          }
//          LinkedList {
//              head: Some(new_node)
//          }
//      }
//  }
//  #[derive(Debug)]
//  struct LinkedList<T> {
//      head: Option<Rc<LinkedListNode<T>>>
//  }
//  #[derive(Debug)]
//  struct LinkedListNode<T> {
//      next: Option<Rc<LinkedListNode<T>>>,
//      prev: Option<Weak<LinkedListNode<T>>>,
//      data: T
//  }
//  fn main() {
//      let list_of_nums = LinkedList::new().append(1).append(2).append(3);
//      println!("nums: {:?}", list_of_nums);
//  }
///```

fn main(){
    unimplemented!();
}
