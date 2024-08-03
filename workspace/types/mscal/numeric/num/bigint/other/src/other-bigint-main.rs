  #![allow(dead_code, unused_variables, unused_imports)]
 use num::{BigInt};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p other-bigint_bin --bin other-bigint-main```
///
/// ```cargo doc  --package other-bigint_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-bigint_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `128 == (2 ** 7) ==  pow(2,7)`
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


fn main() {
    let mut in1: BigInt = 1.into();
    let mut in2: BigInt = 1.into();
    let mut op1: BigInt;

    for iterations in 0..183 {
        //add the given numbers
        op1 = in1 + &in2;
        //print the result
        println!("{}: {}", iterations + 1, op1);
        //prepare the next iterations
        in1 = in2;
        in2 = op1;
        //repeat
    }
}
