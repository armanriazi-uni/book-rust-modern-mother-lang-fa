#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-for-flowcontrol_bin --bin rust-in-action-for-flowcontrol-main```
///
/// ```cargo doc  --package rust-in-action-for-flowcontrol_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-for-flowcontrol_bin```
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

fn main() {
   let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() // 0.. is important
    {
          if i % 2 == 0 {
             continue; //break
            }
    let item = collection[i];
    println!("Printed:{:?}",&item);

    }
}
