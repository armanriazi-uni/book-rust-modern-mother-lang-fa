#![allow(dead_code, unused_variables)]

/// rust-egg-return-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-return_bin --bin rust-egg-return-bin-ex-3```
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
/// `printing: 13`
/// `printing: 99`
/// `Printed:0`
/// `Printed:1`
/// `Printed:2`
/// `Printed:3`
/// `Printed:4`
/// `Printed:[Some(0), Some(0), Some(0), Some(0), Some(0)]`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
/// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 { // replacement of iter would be i
        let number_to_add: u16 = { ((iter * 5) + 2) / (4 * 16) };
        println!("Printed:{}",iter as usize);
        numbers[iter as usize] = Some(number_to_add);// TO_REPORT_BUG:g slice indices are of type `usize` or ranges of `usize` *BecauseOf(-):iter as usize

    }
    println!("Printed:{:?}",numbers);
}
