#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-thread-mutex_bin --bin packtpub-thread-mutex-main```
///
/// ```cargo doc  --package packtpub-thread-mutex_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-thread-mutex_bin```
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
/// `100`
///
/// ## Example
///
/// ```compile_fail,ignore

use std::sync::{Mutex,Arc};
use std::thread::spawn;
fn main() {
    println!("Hello, world!");
    let m = Arc::new(Mutex::new(32));
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let am= m.clone();
        let j = i;
        handles.push(spawn(move ||{
            let mut p = am.lock().unwrap();
            *p += j;
            println!("j = {} , p = {}" ,j,*p);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Done");
}
