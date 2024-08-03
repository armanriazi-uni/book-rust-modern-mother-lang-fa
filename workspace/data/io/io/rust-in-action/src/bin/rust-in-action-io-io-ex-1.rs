#![allow(dead_code, unused_variables)]

/// rust-in-action-io-io-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-io-io_bin --bin rust-in-action-io-io-ex-1```
///
/// ```cargo doc  --package rust-in-action-io-io_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-io-io_bin```
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
/// `100`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// // ```compile_fail,ignore
///
use std::io;
use std::process;
fn main (){
loop{
    let mut first= String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut first);
    //let a:u32 = first.trim().parse().expect("This is not valid number");//unwrap();
    //unwrap: if this result does not emit an error. it will resolve with the value

    let mut a:u32 = 0;
    match first.trim().parse(){
        Ok(val)=>{
            a=val;
        },
        Err(_err)=>{
            println!("This is not a valid number");
            process::exit(1);
        }
    }
    let mut second= String::new();
    println!("Enter second number:");
    io::stdin().read_line(&mut second);
    let b:u32 = second.trim().parse().expect("This is not valid number");

    let result = sum(a,b);
    println!("{} + {} = {}", a,b,result);
 }
}

fn sum(a:u32, b:u32) -> u32{
    a+b
}
