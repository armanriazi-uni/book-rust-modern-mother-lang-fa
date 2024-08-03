/// grammer-dtolnay
///
/// ## Commands
///
/// ```cargo run -q -p grammer-dtolnay_bin --bin grammer-dtolnay-trait-trailing```
///
/// ```cargo doc  --package grammer-dtolnay_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package grammer-dtolnay_bin```
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
/// `1244`
///
/// ## Example
/// `TODO`
/// ```rust,no_run,ignore,compile_fail
/// 
trait Trait {
    fn p(&self);
}
impl Trait for (u32) {
    fn p(&self) { print!("1"); }
}
impl Trait for (i32,) {
    fn p(&self) { print!("2"); }
}
impl Trait for (u32, u32) {
    fn p(&self) { print!("3"); }
}
impl Trait for (i32, i32,) {
    fn p(&self) { print!("4"); }
}
fn main() {
    (0).p();
    (0,).p();
    (0, 0).p();
    (0, 0,).p();
}

/*
1244
The trailing comma is required in the case of a 1-tuple, (0,), because it disambiguates it from (0) which is identical to 0. However, for larger tuples, it is entirely optional: (i32) is a distinct type from (i32,), but (i32, i32) and (i32, i32,) are the same.

An integral literal 0 can be inferred to be any integer type, but defaults to i32 if insufficient type information is available. (0) is inferred to be a u32 and (0,) is inferred to be a (i32,) because those are respectively the only integral and 1-tuple types with an implementation for Trait.

Since (0, 0) and (0, 0,) have the same type, the output of their p methods must be the same, but Rust needs to somehow choose between the two possible implementations of Trait, namely (u32, u32) and (i32, i32). Since i32 is the default integral type, (i32, i32) is chosen in both cases.
*/