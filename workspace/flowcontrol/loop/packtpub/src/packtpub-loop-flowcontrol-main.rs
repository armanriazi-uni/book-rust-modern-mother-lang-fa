#![allow(dead_code, unused_variables)]
use std::time::{Duration, Instant};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p packtpub-loop-flowcontrol_bin --bin packtpub-loop-flowcontrol-main```
///
/// ```cargo doc  --package packtpub-loop-flowcontrol_bin --message-loopmat short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-loop-flowcontrol_bin```
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
/// `Hello, world!
/// 4
/// 7
/// 8
/// 9
/// 11
/// 14
/// 4
/// 7
/// 8`
///
/// ## Example
/// // ```rust,compile_fail,ignore



fn main() {
    println!("Hello, world!");
    //loop_to_10();
    array_loop();
}

fn loop_to_10(){
    for n in 0..10 {
        println!("Hello {}",n);
    }
}

fn array_loop(){

    let v = vec![4,7,8,9,11,14];

    'outer: for i in 1..10 {
        for n in &v {
            if i+n ==11  {
                break 'outer;
            }
            println!("{}",n);
        }
    }
}
