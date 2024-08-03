#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-mpsc_bin --bin rust-in-action-thread-mpsc-main```
///
/// ```cargo doc  --package rust-in-action-thread-mpsc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-mpsc_bin```
///
/// ## What
/// `sleeping_subthread`
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

use std::{thread, time};
 fn main() {
   let start = time::Instant::now();
   let handler_1 = thread::spawn(move || {
     let pause = time::Duration::from_millis(300);
       thread::sleep(pause.clone());
   });
   let handler_2 = thread::spawn(move || {
     let pause = time::Duration::from_millis(300);
       thread::sleep(pause.clone());
   });
   let handler_3 = thread::spawn(move || {
    let pause = time::Duration::from_millis(300);
      thread::sleep(pause.clone());
  });
   handler_1.join().unwrap();
   handler_2.join().unwrap();
   handler_3.join().unwrap();
   let finish = time::Instant::now();
   println!("{:?}", finish.duration_since(start));
 }
/*

    The program starts by recording the current instant in time (start) using time::Instant::now().

    Three threads are spawned, each sleeping for 300 milliseconds. These threads are represented by handler_1, handler_2, and handler_3.

    The main thread waits for each of the spawned threads to complete their execution using join(). This ensures that the main thread will only proceed after all threads have finished.

    Once all threads have completed, the program records another instant in time (finish) and calculates the duration since the start of the program using finish.duration_since(start).

    Finally, the program prints the calculated duration, showing the total time taken for all threads to complete their execution.

*/