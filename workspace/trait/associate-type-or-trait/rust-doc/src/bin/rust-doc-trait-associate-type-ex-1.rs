#![allow(dead_code, unused_variables)]

use std::rc::Rc;

/// rust-doc-trait-associate-type-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-associate-type-and-trait_bin --bin  rust-doc-trait-associate-type-ex-1```
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
/// //``rust,no_run,compile_fail,ignore

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/* With generic type substitute associated type

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

*/

fn main(){
    unimplemented!()
}
