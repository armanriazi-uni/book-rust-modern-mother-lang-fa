#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-slice_bin --bin rust-doc-slice-main```
///
/// ```cargo doc  --package rust-doc-slice_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-slice_bin```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //rust,compile_fail,no_run,ignore
// In return because we do not have Type like String so we can dangling problem
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let binding = String::from("Arman Riazi");
    let a=first_word(&binding);
    println!("Printed:{:?}",a);

}
