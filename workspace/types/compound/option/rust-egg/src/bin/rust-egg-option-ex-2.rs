#![allow(dead_code, unused_variables)]

/// rust-egg-option-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-option_bin --bin rust-egg-option-ex-2```
///
/// ```cargo doc  --package rust-egg-option_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-option_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your option/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  // ```rust,compile_fail,ignore

/// Make me compile! Execute `rustlings hint option2` for hints
fn main(){
    let mut res=42;
    let option = Some(12);
    if let Some(x)=option{
        res+=x;
    }
    /* no clippy code
    for x in option{
        res+=x;
    }*/
    println!("Printed:{}",res);
}
