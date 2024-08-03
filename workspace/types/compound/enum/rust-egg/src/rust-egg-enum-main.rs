#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-enum_bin --bin rust-egg-enum-ex-1```
///
/// ```cargo doc --package rust-egg-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package rust-egg-enum_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your enum/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore

#[derive(Debug)]
enum Message {
    //classic : c-like structures.
    Move{x:u32,y:u32},
    Echo(String),
    ChangeColor(u8,u8,u8),
    //`unit-like`,
    Quit
}

impl Message {
    fn call(&self){
        println!("{:?}",&self);
    }
}

fn main(){
    let messages = [
        Message::Move{ x: 10, y:30},
        Message::Echo(String::from("hello")),
        Message::ChangeColor(200,255,255),
        Message::Quit
    ];

    for msg in & messages {
        msg.call();
    }
}
