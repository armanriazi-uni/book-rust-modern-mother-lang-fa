#![allow(dead_code, unused_variables)]
/// easyrust-closure-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p easyrust-closure_bin --bin easyrust-closure-ex-4```
///
/// ```cargo doc  --package easyrust-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package easyrust-closure_bin ```
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


fn main() {
    let mut my_string = String::from("Can I go inside the thread?");

    let handle = std::thread::spawn(move || {
        println!("{}", my_string); // now my_string is being used as a reference
    });

   // std::mem::drop(my_string);  // ⚠️ We try to drop it here. But the thread still needs it.//we have better use {} instead of drop

    handle.join();
}

// fn main(){
//     unimplemented!();
// }
