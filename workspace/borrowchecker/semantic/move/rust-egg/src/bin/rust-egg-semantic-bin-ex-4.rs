#![allow(dead_code, unused_variables)]

/// rust-egg-dangling-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-semantic_bin --bin  rust-egg-semantic-bin-ex-4```
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
/// //```rust,no_run,compile_fail,ignore
fn main(){
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
//let r3 = &mut s; 

 //println!("{}, {}, and {}", r1, r2, r3);

//Resolved:
/*let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);*/
}
