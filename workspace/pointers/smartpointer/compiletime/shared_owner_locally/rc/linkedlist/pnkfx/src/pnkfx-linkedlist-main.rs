#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p pnkfx-linkedlist_bin --bin pnkfx-linkedlist-main```
///
/// ```cargo doc  --package pnkfx-linkedlist_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pnkfx-linkedlist_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// EXERCISE: The code does not compile. There is something
/// fundamentally wrong with the "desired" API above. Come
/// up with a fix.
///
/// HINT: Review first exercise from ex10.
/// EXERCISE: Add an additional `head_and_tail` method that follows
/// through on the original intent of `fn head`.
/// EXERCISE: Inspect the size_of the HeavyStack<int> type.
///
/// Can you explain where every byte comes from?
///
/// Can you make an alternative API what is more size-efficient?
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
// extern crate arena;
// use arena::{TypedArena, Arena};

// #[derive(Clone,Debug)]
// pub struct HsNode {
//     arena: TypedArena<HeavyStack>,
//     value: i32, // Assuming X is i32 for simplicity; adjust as needed
//     next: HeavyStack,
// }
// #[derive(Clone,Debug)]
// pub struct HsLeaf {
//     arena: TypedArena<HeavyStack>,
// }
// #[derive(Clone,Debug)]
// pub enum HeavyStack {
//     Cons(HsNode),
//     Null(HsLeaf),
// }

// trait StackMaker {
//     fn make_null(&self) -> &HeavyStack;
//     fn make_cons(&self, x: i32, t: &HeavyStack) -> &HeavyStack;
// }

// impl StackMaker for TypedArena<HeavyStack> {
//     fn make_null(&self) -> &HeavyStack {
//         self.alloc(HeavyStack::Null(HsLeaf { arena: self.clone() }))
//     }

//     fn make_cons(&self, x: i32, t: &HeavyStack) -> &HeavyStack {
//         self.alloc(HeavyStack::Cons(HsNode { arena: self.clone(), value: x, next: t.clone() }))
//     }
// }

// pub type HS = &HeavyStack;

// impl HeavyStack {
//     pub fn is_empty(&self) -> bool {
//         matches!(*self, HeavyStack::Null(_))
//     }

//     pub fn arena(&self) -> &TypedArena<HeavyStack> {
//         match *self {
//             HeavyStack::Cons(ref node) => &node.arena,
//             HeavyStack::Null(ref leaf) => &leaf.arena,
//         }
//     }

//     pub fn empty(arena: &TypedArena<HeavyStack>) -> HS {
//         arena.make_null()
//     }

//     pub fn cons(&self, x: i32) -> HS {
//         self.arena().make_cons(x, self)
//     }

//     pub fn tail(&self) -> Option<&HeavyStack> {
//         match self {
//             &HeavyStack::Null(_) => None,
//             &HeavyStack::Cons(ref n) => Some(&n.next),
//         }
//     }

//     pub fn head(&self) -> Option<i32> {
//         match self {
//             &HeavyStack::Null(_) => None,
//             &HeavyStack::Cons(n) => Some(n.value),
//         }
//     }
// }

// fn main() {
//     let r = TypedArena::<HeavyStack>::new();
//     let s0a = HeavyStack::empty(&r); // ()
//     assert!(s0a.is_empty());

//     let s1b = s0a.cons(1); // (1)
//     assert_eq!(s1b.head(), Some(1));
//     println!("s1b.head: {:?}", s1b.head());
// }
fn main() {
    unimplemented!();
}