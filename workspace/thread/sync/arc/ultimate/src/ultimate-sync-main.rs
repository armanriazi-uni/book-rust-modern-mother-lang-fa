#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p ultimate-sync-arc_bin --bin ultimate-sync-main```
///
/// ```cargo doc  --package ultimate-sync-arc_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package ultimate-sync-arc_bin```
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

///
/// ```compile_fail,ignore
/// 
use std::thread;
fn main() {
  let data = vec![1, 2, 3, 4, 5];
  let shared_data = std::sync::Arc::new(data);
  let handles: Vec<_> = (0..5).map(|i| {
  let shared_data = shared_data.clone();
    thread::spawn(move || {
      let local_sum: i32 = shared_data.iter().sum();
      println!("Thread {} Sum: {}", i, local_sum);
    })
}).collect();
  for handle in handles {
    handle.join().unwrap();
  }
}