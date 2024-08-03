#![allow(dead_code, unused_variables, unused_imports)]
use num::{FromPrimitive,BigInt};
use num::rational::{Ratio, BigRational};
/// Main
///
/// # Commands
///
/// ```cargo run -q -p other-bigrational_bin --bin other-bigrational-main```
///
/// ```cargo doc  --package other-bigrational_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-bigrational_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`



fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}

fn main() {
    println!("{}", approx_sqrt(10, 4)); // prints 4057691201/1283082416
}
