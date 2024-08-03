#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-enum_bin --bin packtpub-enum-main```
///
/// ```cargo doc  --package packtpub-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-enum_bin```
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
pub struct Bed{
    size:i32,
    count:u32,
}
#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge(i32,String),
}
fn main() {
    use self::Room::*;
    let t = Lounge(5,"big".to_string());
    println!("Hello from the {:?}",t);

    if let Lounge(n,s) = t {
        println!("Its a {} lounge with {} cupboards",s,n);
    }
}
