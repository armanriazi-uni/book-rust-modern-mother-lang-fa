#![allow(dead_code, unused_variables)]


/// rust-doc-trait-dispatchable-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-dispatchable_bin --bin  rust-doc-trait-dispatchable-ex-6```
///
/// ## What
/// `Is not-allow-borowchecker-messeng`
///
/// ## How
/// `This ex is comparable by ex-5 that used &self`
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
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    //println!("A baby dog is called a {}", Animal::baby_name()); //error cannot call associated function of trait-Rust can’t figure out which implementation of Animal::baby_name we want

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
