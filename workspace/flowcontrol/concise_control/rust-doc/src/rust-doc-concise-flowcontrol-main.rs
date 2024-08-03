#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-concise-flowcontrol_bin --bin rust-doc-concise-flowcontrol-main```
///
/// ```cargo doc  --package rust-doc-concise-flowcontrol_bin --message-concisemat short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-concise-flowcontrol_bin```
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
/// `nothing`
///
/// ## Example
/// // ```compile_fail,ignore
/// //concise_control:
/// Coin::Quarter(state) => println!("State quarter from {:?}!", state),
/// if let Some(max) = config_max {}
/// ```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
