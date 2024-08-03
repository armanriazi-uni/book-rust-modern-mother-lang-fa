#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// master-rust-trait-bound-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-trait-bound_bin --bin  master-rust-trait-bound-ex-1```
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
