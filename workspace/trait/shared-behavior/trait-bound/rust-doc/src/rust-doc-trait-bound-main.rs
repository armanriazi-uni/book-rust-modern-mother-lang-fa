#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// rust-doc--trait-bound-main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-bound_bin --bin  rust-doc-trait-bound-main```
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

trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

impl<T> Eatable for Food<T> where T: Debug {
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

fn eat<T>(val: T) where T: Eatable {
    val.eat();
}

fn main() {
    let apple = Food(Apple);
    eat(apple);
}
