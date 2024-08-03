#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p easy-rust-smartpointer-ops_bin --bin easy-rust-smartpointer-ops-main```
///
/// ```cargo doc  --package easy-rust-smartpointer-ops_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package easy-rust-smartpointer-ops_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `This works as you would expect, and the only added cost is that you have to write a bit more. The additional runtime cost is zero, though, and the references to the mutable things remain immutable.`
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
/// //```rust,compile_fail,no_run,ignore
use std::ops::{Deref, DerefMut};
struct HoldsANumber(u8);
impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}
impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
fn main() {
    let mut my_number = HoldsANumber(20);
    *my_number = 30; // DerefMut lets us do this
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
}
