#![allow(dead_code, unused_variables)]
///
/// rust-doc-concise-flowcontrol-bin-ex-1
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-concise-flowcontrol_bin --bin rust-doc-concise-flowcontrol-bin-ex-1```
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
///
/// //```rust,compile_fail,no_run
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
