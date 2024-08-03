#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

use std::fmt::Display;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p ultimate-generic_bin --bin ultimate-generic-main```
///
/// ```cargo doc  --package ultimate-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package ultimate-generic_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// 
/// ## Example
/// //```rust,compile_fail,ignore
///
// Example 11: enums


struct Stack<T> {
    items: Vec<T>,
    }
    impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);
        }
        fn pop(&mut self) -> Option<T> { 
            self.items.pop()
        }
}
    fn main() {    
        let mut stack = Stack::<i32> { items: Vec::new() };
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        while let Some(item) = stack.pop() {
            println!(“Popped: {}”, item);
        }
}
// Output 'static value passed in is: 5