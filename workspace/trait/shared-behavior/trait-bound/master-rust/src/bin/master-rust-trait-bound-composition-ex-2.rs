#![allow(dead_code, unused_variables)]
use std::fmt::Debug;


/// master-rust-trait-bound-composition-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-trait-bound_bin --bin  master-rust-trait-bound-composition-ex-2```
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
trait Eat {
    fn eat(&self) {
        println!("eat");
    }
}
trait Code {
    fn code(&self) {
        println!("code");
    }
}
trait Sleep {
    fn sleep(&self) {
        println!("sleep");
    }
}

trait Programmer : Eat + Code + Sleep {
    fn animate(&self) {
        self.eat();
        //self.code();
        self.sleep();
        println!("repeat!");
    }
}

struct Bob;
impl Programmer for Bob {}
impl Eat for Bob {}
impl Code for Bob {}
impl Sleep for Bob {}

fn main() {
    Bob.animate();
}
