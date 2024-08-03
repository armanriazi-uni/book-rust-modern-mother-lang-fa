#![allow(dead_code, unused_variables)]

/// rust-egg-return-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-return_bin --bin rust-egg-return-bin-ex-1```
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
/// `that is color`
///
/// ## Example
/// //```rust,compile_fail,ignore
///

fn main(){

    let word=String::from("green");
    if iscolor(&word){
        println!("that is color");
    }
    else{
        println!("that is no color");
    }
}

fn iscolor(attempt: &str)-> bool{
attempt == "green" || attempt == "red" || attempt=="blue"
}
