#![allow(dead_code, unused_variables)]


/// rust-in-action-duplication-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-duplication_bin --bin  rust-in-action-duplication-ex-1```
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
/// //``rust,no_run,compile_fail,ignore
#[derive(PartialEq)] // <1>
struct Hostname(String); // <2>

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname ( ordinary_string.clone() );
    if host.0 == ordinary_string { // <3>
      println!("huh?{}",{ordinary_string});
    };
}
