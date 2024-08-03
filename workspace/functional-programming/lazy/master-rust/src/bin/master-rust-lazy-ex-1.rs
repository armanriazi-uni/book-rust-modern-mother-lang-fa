#![allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use std::sync::Mutex;
/// master-rust-lazy-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-lazy_bin --bin master-rust-lazy-ex-1```
///
/// ```cargo doc  --package master-rust-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-lazy_bin ```
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

lazy_static! {
    static ref ITEMS: Mutex<Vec<u64>> = {
        let mut v = vec![];
        v.push(9);
        v.push(2);
        v.push(1);
        Mutex::new(v)
    };
}
fn main(){
       ////First access to `ITEMS` initializes it
    let mut num = ITEMS.lock().unwrap();
    println!("The entry for `0` is \"{:?}\".", num);
 
}
