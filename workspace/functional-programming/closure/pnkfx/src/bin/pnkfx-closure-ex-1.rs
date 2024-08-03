#![allow(dead_code, unused_variables)]
/// pnkfx-closure-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p pnkfx-closure_bin --bin pnkfx-closure-ex-1```
///
/// ```cargo doc  --package pnkfx-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pnkfx-closure_bin ```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
//#![feature(fn_traits)]
//#![feature(unboxed_closures)]
//#[cfg(feature = "unboxed_closures")]
use std::ops::FnOnce;
struct Closure<'a> {
    s: String,
    t: &'a String,
}

impl<'a> Closure<'a> {
    fn new(s: String, t: &'a String) -> Self {
        Closure { s, t }
    }

    fn call(&mut self) -> String {
        println!("call_mut()");
        self.modify_s();
        self.s.clone()
    }

    fn call_once(self) -> String {
        println!("call_once()");
        self.s
    }

    fn modify_s(&mut self) {
        self.s.push_str(&self.t[..]);
    }
}

//extern "rust-call"
fn add_args(args: (u32, u32)) -> u32 {
    args.0 + args.1
}
fn main() {
    unimplemented!();
    // Example strings
    // let string1 = "Hello".to_string();
    // let string2 =   &String::from("World");
    // // Creating a Closure instance
    // let mut closure_instance = Closure::new(string1.clone(), &String::from(&string2));
    // // Demonstrating call_mut() - modifying the instance
    // let result_call_mut = closure_instance.call();
    // println!("Result of call_mut(): {}", result_call_mut);
    // // Demonstrating call_once() - not modifying the instance
    // let result_call_once = closure_instance.call_once();
    // println!("Result of call_once(): {}", result_call_once);
    // // Example usage of add_args function
    // let sum = add_args((10, 20));
    // println!("Sum of 10 and 20 is: {}", sum);
}