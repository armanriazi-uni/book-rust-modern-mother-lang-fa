#![allow(dead_code, unused_variables)]



/// master-rust-trait-associate-type-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-trait-associate-type_bin --bin  master-rust-trait-associate-type-ex-1```
///
/// ## What
/// `trait_constants`
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

trait Circular {
    //const PI: f64 = 3.14;
    fn area(&self) -> f64;
}

struct Circle {
    rad: f64
}

impl Circular for Circle {
    fn area(&self) -> f64 {
         std::f64::consts::PI * self.rad * self.rad
    }
}

fn main() {
    let c_one = Circle { rad: 4.2 };
    let c_two = Circle { rad: 75.2 };
    println!("Area of circle one: {}", c_one.area());
    println!("Area of circle two: {}", c_two.area());
}
