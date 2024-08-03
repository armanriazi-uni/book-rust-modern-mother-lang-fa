#![allow(dead_code, unused_variables)]

/// rust-in-action-int-bin-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-int_bin --bin rust-in-action-int-bin-ex-4```
///
/// ## What
/// `loop int`
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
/// //```rust,compile_fail,ignore

fn main() {
    let mut i: u16 = 0;
    print!("{:}..", i);
    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            print! {"Mod Equal zero\n"}
        }
    }
}
