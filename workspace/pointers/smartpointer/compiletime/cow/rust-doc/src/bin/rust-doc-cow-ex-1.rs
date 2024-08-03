#![allow(dead_code, unused_variables)]
use std::borrow::Cow;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-cow_bin --bin rust-doc-cow-ex-1```
///
/// ```cargo doc  --package rust-doc-cow_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-cow_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  `TODO`
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
///  `TODO`

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {   
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
            println!("Clone occured!");
        }
    }
    println!("{:?}",input);
}
fn main() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    // No clone occurs because `input` is already owned.
    //let mut input = Cow::from(& mut input);
    abs_all(&mut input);
}
