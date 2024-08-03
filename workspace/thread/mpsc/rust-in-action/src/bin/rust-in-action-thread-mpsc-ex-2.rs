#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-mpsc_bin --bin rust-in-action-thread-mpsc-ex-2```
///
/// ```cargo doc  --package rust-in-action-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-mpsc_bin```
///
/// ## What
/// `busywait_spin_strategy`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothig`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore
use std::{thread, time};
 fn main() {
   for n in 1..101 {
     let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
     let start = time::Instant::now();
     for _m in 0..n {
       let handle = thread::spawn(|| {
         let start = time::Instant::now();
         let pause = time::Duration::from_millis(20);
         while start.elapsed() < pause {
           thread::yield_now();
         }
       });
       handlers.push(handle);
     }
     while let Some(handle) = handlers.pop() {
       handle.join();
     }
     let finish = time::Instant::now();
     println!("{}\t{:02?}", n, finish.duration_since(start));
   }
 }
