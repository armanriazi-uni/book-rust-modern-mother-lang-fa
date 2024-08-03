#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-option_bin --bin rust-egg-option-main```
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
// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}
fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 5) + 2) / (4 * 16) };
        println!("Printed:{}",iter as usize);
        numbers[iter as usize] = Some(number_to_add);// TO_REPORT_BUG:g slice indices are of type `usize` or ranges of `usize` *BecauseOf(-):iter as usize
    }
    println!("Printed:{:?}",numbers);

}
